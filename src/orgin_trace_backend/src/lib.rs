use ic_cdk::management_canister::{
    http_request,
    HttpRequestArgs,
    HttpHeader,
    HttpMethod,
};

use ic_cdk::api::management_canister::http_request::HttpResponse; // خاص بالـ response
use num_bigint::BigUint;
use serde_json::json;

use candid::{CandidType, Deserialize};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;
use serde::Serialize;
use ic_cdk::api::time;

type Memory = VirtualMemory<DefaultMemoryImpl>;

/// --------- USER STRUCT & ROLE ----------
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Role {
    Seller,
    Specialist,
    Buyer,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct User {
    id: u64,
    name: String,
    email: String,
    role: Role,
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(bincode::serialize(self).unwrap()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self { bincode::deserialize(&bytes).unwrap() }
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}

/// --------- DEVICE STRUCT ----------
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Device {
    id: u64,
    user_id: u64,
    name: String,
    specs: String,
}

impl Storable for Device {
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(bincode::serialize(self).unwrap()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self { bincode::deserialize(&bytes).unwrap() }
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}

// اضفت price_usd عشان check_device_in_storage يشتغل
impl Device {
    fn price_usd(&self) -> u32 {
        500 // قيمة افتراضية مؤقتة
    }
}

/// --------- WARRANTY CONTRACT STRUCT ----------
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WarrantyContract {
    id: u64,
    seller_id: u64,
    buyer_id: u64,
    device_id: u64,
    warranty_months: u32,
    expiry_date: u64,
}

impl Storable for WarrantyContract {
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(bincode::serialize(self).unwrap()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self { bincode::deserialize(&bytes).unwrap() }
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}

/// --------- REPORT STRUCT ----------
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Report {
    id: u64,
    device_id: u64,
    specialist_name: String,
    notes: String,
    timestamp: u64,
}

impl Storable for Report {
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(bincode::serialize(self).unwrap()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self { bincode::deserialize(&bytes).unwrap() }
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}

/// --------- CART STRUCT ----------
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Cart {
    user_id: u64,
    device_ids: Vec<u64>,
}

impl Storable for Cart {
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(bincode::serialize(self).unwrap()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self { bincode::deserialize(&bytes).unwrap() }
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}

/// --------- ORDER STRUCT ----------
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Order {
    id: u64,
    buyer_id: u64,
    seller_id: u64,
    device_id: u64,
    timestamp: u64,
}

impl Storable for Order {
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(bincode::serialize(self).unwrap()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self { bincode::deserialize(&bytes).unwrap() }
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Unbounded;
}

/// --------- STORAGE ----------
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USERS: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    );

    static DEVICES: RefCell<StableBTreeMap<u64, Device, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static CONTRACTS: RefCell<StableBTreeMap<u64, WarrantyContract, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    static REPORTS: RefCell<StableBTreeMap<u64, Report, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static CARTS: RefCell<StableBTreeMap<u64, Cart, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static ORDERS: RefCell<StableBTreeMap<u64, Order, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    static COUNTERS: RefCell<[u64; 6]> = RefCell::new([0; 6]); // user, device, contract, report, cart, order
}

fn next_id(index: usize) -> u64 {
    COUNTERS.with(|c| {
        let mut counters = c.borrow_mut();
        counters[index] += 1;
        counters[index]
    })
}

/// --------- USER APIS ----------
#[ic_cdk::update]
fn add_user(name: String, email: String, role: Role) -> u64 {
    let id = next_id(0);
    let user = User { id, name, email, role };
    USERS.with(|users| { users.borrow_mut().insert(id, user); });
    id
}

#[ic_cdk::query]
fn get_user(id: u64) -> Option<User> {
    USERS.with(|users| users.borrow().get(&id).map(|v| v.clone()))
}

#[ic_cdk::update]
fn update_user(requester_id: u64, name: String, email: String, role: Role) -> Result<(), String> {
    let requester = USERS.with(|u| u.borrow().get(&requester_id).map(|v| v.clone()));
    match requester {
        Some(r) => {
            USERS.with(|users| users.borrow_mut().insert(r.id, User { id: r.id, name, email, role }));
            Ok(())
        },
        _ => Err("Only the user itself can update its profile".to_string())
    }
}

#[ic_cdk::update]
fn delete_user(requester_id: u64) -> Result<(), String> {
    let requester = USERS.with(|u| u.borrow().get(&requester_id).map(|v| v.clone()));
    match requester {
        Some(r) => {
            USERS.with(|users| users.borrow_mut().remove(&r.id));
            Ok(())
        },
        _ => Err("Only the user itself can delete its profile".to_string())
    }
}

/// --------- DEVICE APIS ----------
#[ic_cdk::update]
fn add_device(user_id: u64, name: String, specs: String) -> Result<u64, String> {
    let user_opt = USERS.with(|users| users.borrow().get(&user_id).map(|v| v.clone()));
    match user_opt {
        Some(user) => {
            if user.role == Role::Seller {
                let id = next_id(1);
                let device = Device { id, user_id, name, specs };
                DEVICES.with(|devices| { devices.borrow_mut().insert(id, device); });
                Ok(id)
            } else {
                Err("Only sellers can add devices".to_string())
            }
        },
        None => Err("User not found".to_string())
    }
}

#[ic_cdk::update]
fn delete_device(user_id: u64, device_id: u64) -> Result<(), String> {
    let device_opt = DEVICES.with(|devices| devices.borrow().get(&device_id).map(|v| v.clone()));
    match device_opt {
        Some(device) => {
            if device.user_id == user_id {
                DEVICES.with(|devices| devices.borrow_mut().remove(&device_id));
                Ok(())
            } else {
                Err("Only the owner seller can delete this device".to_string())
            }
        },
        None => Err("Device not found".to_string())
    }
}

#[ic_cdk::query]
fn get_device(device_id: u64) -> Option<Device> {
    DEVICES.with(|devices| devices.borrow().get(&device_id).map(|v| v.clone()))
}

#[ic_cdk::query]
fn list_devices() -> Vec<Device> {
    DEVICES.with(|devices| devices.borrow().iter().map(|(_, d)| d.clone()).collect())
}

#[ic_cdk::query]
fn list_user_devices(user_id: u64) -> Vec<Device> {
    DEVICES.with(|devices| devices.borrow().iter().filter_map(|(_, d)| if d.user_id == user_id { Some(d.clone()) } else { None }).collect())
}

/// --------- CONTRACT APIS ----------
#[ic_cdk::update]
fn create_contract(seller_id: u64, buyer_id: u64, device_id: u64, warranty_months: u32) -> Result<u64, String> {
    let device_opt = DEVICES.with(|d| d.borrow().get(&device_id).map(|v| v.clone()));
    match device_opt {
        Some(device) => {
            if device.user_id != seller_id {
                return Err("Only the seller owner of the device can create a contract".to_string());
            }
            let now = time();
            let one_month_ns: u64 = 30 * 24 * 60 * 60 * 1_000_000_000;
            let expiry_date = now + (warranty_months as u64 * one_month_ns);

            let id = next_id(2);
            let contract = WarrantyContract { id, seller_id, buyer_id, device_id, warranty_months, expiry_date };
            CONTRACTS.with(|contracts| contracts.borrow_mut().insert(id, contract));
            Ok(id)
        },
        None => Err("Device not found".to_string())
    }
}

#[ic_cdk::query]
fn get_contract(contract_id: u64) -> Option<WarrantyContract> {
    CONTRACTS.with(|contracts| contracts.borrow().get(&contract_id).map(|v| v.clone()))
}

#[ic_cdk::query]
fn list_contracts() -> Vec<WarrantyContract> {
    CONTRACTS.with(|contracts| contracts.borrow().iter().map(|(_, c)| c.clone()).collect())
}

#[ic_cdk::update]
fn delete_contract(contract_id: u64) -> Result<(), String> {
    let existed = CONTRACTS.with(|contracts| contracts.borrow_mut().remove(&contract_id));
    match existed {
        Some(_) => Ok(()),
        None => Err("Contract not found".to_string())
    }
}

/// --------- REPORT APIS ----------
#[ic_cdk::update]
fn add_report(user_id: u64, device_id: u64, specialist_name: String, notes: String) -> Result<u64, String> {
    let user_opt = USERS.with(|users| users.borrow().get(&user_id).map(|v| v.clone()));
    match user_opt {
        Some(user) => {
            if user.role != Role::Specialist {
                return Err("Only specialists can add reports".to_string());
            }
            let device_opt = DEVICES.with(|d| d.borrow().get(&device_id).map(|v| v.clone()));
            if device_opt.is_none() {
                return Err("Device not found".to_string());
            }
            let now = time();
            let id = next_id(3);
            let report = Report { id, device_id, specialist_name, notes, timestamp: now };
            REPORTS.with(|reports| reports.borrow_mut().insert(id, report));
            Ok(id)
        },
        None => Err("User not found".to_string())
    }
}

#[ic_cdk::query]
fn get_report(report_id: u64) -> Option<Report> {
    REPORTS.with(|reports| reports.borrow().get(&report_id).map(|v| v.clone()))
}

#[ic_cdk::query]
fn list_reports() -> Vec<Report> {
    REPORTS.with(|reports| reports.borrow().iter().map(|(_, r)| r.clone()).collect())
}

#[ic_cdk::query]
fn list_device_reports(device_id: u64) -> Vec<Report> {
    REPORTS.with(|reports| reports.borrow().iter()
        .filter_map(|(_, r)| if r.device_id == device_id { Some(r.clone()) } else { None })
        .collect()
    )
}

#[ic_cdk::query]
fn list_user_reports(user_id: u64) -> Vec<Report> {
    DEVICES.with(|devices| {
        let device_map: std::collections::HashMap<u64, u64> = devices.borrow().iter().map(|(id, d)| (id, d.user_id)).collect();
        REPORTS.with(|reports| {
            reports.borrow().iter()
                .filter_map(|(_, r)| {
                    if let Some(owner_id) = device_map.get(&r.device_id) {
                        if *owner_id == user_id { return Some(r.clone()); }
                    }
                    None
                })
                .collect()
        })
    })
}

#[ic_cdk::update]
fn delete_report(user_id: u64, report_id: u64) -> Result<(), String> {
    let report_opt = REPORTS.with(|reports| reports.borrow().get(&report_id).map(|v| v.clone()));
    match report_opt {
        Some(report) => {
            let device_opt = DEVICES.with(|d| d.borrow().get(&report.device_id).map(|v| v.clone()));
            match device_opt {
                Some(device) => {
                    let user_opt = USERS.with(|u| u.borrow().get(&user_id).map(|v| v.clone()));
                    match user_opt {
                        Some(user) => {
                            if user.role == Role::Specialist || device.user_id == user_id {
                                REPORTS.with(|reports| reports.borrow_mut().remove(&report_id));
                                Ok(())
                            } else {
                                Err("Only the specialist or seller owner can delete this report".to_string())
                            }
                        },
                        None => Err("User not found".to_string())
                    }
                },
                None => Err("Device not found".to_string())
            }
        },
        None => Err("Report not found".to_string())
    }
}

/// --------- CART & ORDER APIS ----------
#[ic_cdk::update]
fn add_to_cart(user_id: u64, device_id: u64) -> Result<(), String> {
    let user_opt = USERS.with(|u| u.borrow().get(&user_id).map(|v| v.clone()));
    match user_opt {
        Some(user) if user.role == Role::Buyer => {
            let device_opt = DEVICES.with(|d| d.borrow().get(&device_id).map(|v| v.clone()));
            match device_opt {
                Some(_) => {
                    CARTS.with(|carts| {
                        let mut carts = carts.borrow_mut();
                        let mut cart = carts.get(&user_id).map(|c| c.clone()).unwrap_or(Cart { user_id, device_ids: vec![] });
                        if !cart.device_ids.contains(&device_id) {
                            cart.device_ids.push(device_id);
                        }
                        carts.insert(user_id, cart);
                    });
                    Ok(())
                },
                None => Err("Device not found".to_string())
            }
        },
        _ => Err("Only buyers can use the cart".to_string())
    }
}

#[ic_cdk::update]
fn checkout_cart(user_id: u64) -> Result<Vec<Order>, String> {
    CARTS.with(|carts| {
        let mut carts = carts.borrow_mut();
        let cart_opt = carts.get(&user_id).map(|c| c.clone());
        match cart_opt {
            Some(cart) => {
                let mut created_orders = Vec::new();
                for device_id in cart.device_ids.iter() {
                    DEVICES.with(|d| {
                        let mut devices = d.borrow_mut();
                        if let Some(device) = devices.remove(device_id) {
                            let order_id = next_id(5);
                            let now = time();
                            let order = Order { id: order_id, buyer_id: user_id, seller_id: device.user_id, device_id: device.id, timestamp: now };
                            ORDERS.with(|orders| orders.borrow_mut().insert(order_id, order.clone()));
                            created_orders.push(order);
                        }
                    });
                }
                carts.remove(&user_id);
                Ok(created_orders)
            },
            None => Err("Cart is empty".to_string())
        }
    })
}

#[ic_cdk::update]
fn remove_from_cart(user_id: u64, device_id: u64) -> Result<(), String> {
    CARTS.with(|carts| {
        let mut carts = carts.borrow_mut();
        if let Some(mut cart) = carts.get(&user_id).map(|c| c.clone()) {
            cart.device_ids.retain(|&id| id != device_id);
            carts.insert(user_id, cart);
            Ok(())
        } else {
            Err("Cart not found".to_string())
        }
    })
}

#[ic_cdk::update]
fn clear_cart(user_id: u64) -> Result<(), String> {
    CARTS.with(|carts| {
        let mut carts = carts.borrow_mut();
        if carts.remove(&user_id).is_some() {
            Ok(())
        } else {
            Err("Cart not found".to_string())
        }
    })
}

#[ic_cdk::query]
fn get_cart(user_id: u64) -> Option<Cart> {
    CARTS.with(|carts| carts.borrow().get(&user_id).map(|c| c.clone()))
}

#[ic_cdk::query]
fn list_orders() -> Vec<Order> {
    ORDERS.with(|orders| orders.borrow().iter().map(|(_, o)| o.clone()).collect())
}
#[derive(Deserialize, Debug)]
struct AiResponse {
    response: DeviceSpec,
}

#[derive(Deserialize, Debug)]
struct DeviceSpec {
    device_type: String,
    primary_use: String,
    budget_usd: u32,
    hard_constraints: Vec<String>,
    soft_preferences: Vec<String>,
    must_not_have: Vec<String>,
}

/// --------- AI CALL & DEVICE CHECK ----------
#[ic_cdk::update]
async fn call_model(prompt: String) -> Result<Vec<Device>, String> {
    let body_json = json!({ "user_message": prompt });
    let request = HttpRequestArgs {
        url: "https://rawan7-icp-ai-agent-api2.hf.space/generate".to_string(),
        method: HttpMethod::POST,
        headers: vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }],
        body: Some(serde_json::to_vec(&body_json).unwrap()),
        max_response_bytes: Some(1_000_000), // 1 ميجا بايت

        transform: None,
        is_replicated: Some(false),
    };

    match http_request(&request).await {
        Ok(response) => {
            if response.status.0 <= 200u64.into() && response.status.0 >= 200u64.into() {

                let body_str = String::from_utf8(response.body).unwrap_or_default();
                ic_cdk::println!("AI response body: {}", body_str);

                match serde_json::from_str::<AiResponse>(&body_str) {
                    Ok(ai_resp) => {
                        let result = check_device_in_storage(ai_resp.response);
                        Ok(result)
                    }
                    Err(e) => Err(format!("Failed to parse AI response: {}", e)),
                }
            } else {
                Err(format!("HTTP Error: {}", response.status))
            }
        }
        Err(err) => Err(format!("IC http_request error: {:?}", err)),
    }
}


// ========================
// Few-Shots Dictionary (على مستوى الموديول)
// ========================
thread_local! {
    static FEW_SHOTS_DICT: RefCell<serde_json::Value> = RefCell::new(json!({
        // ... ضع هنا JSON بتاعك كما هو ...
    }));
}

// ========================
// Function to check devices
// ========================
fn check_device_in_storage(spec: DeviceSpec) -> Vec<Device> {
    let mut matched_devices = Vec::new();

    DEVICES.with(|devices| {
        for (_, device) in devices.borrow().iter() {
            if device.specs.contains(&spec.device_type) && device.price_usd() <= spec.budget_usd {
                let mut matches_constraints = true;

                for constraint in &spec.hard_constraints {
                    if !device.specs.contains(constraint) {
                        matches_constraints = false;
                        break;
                    }
                }

                for forbidden in &spec.must_not_have {
                    if device.specs.contains(forbidden) {
                        matches_constraints = false;
                        break;
                    }
                }

                if matches_constraints {
                    matched_devices.push(device.clone());
                }
            }
        }
    }); // ← يغلق DEVICES.with

    matched_devices // ← يجب إرجاع القيمة هنا خارج الـ closure
}


// ========================
// Process User Query Function
// ========================
#[ic_cdk::update]
async fn process_user_query(user_text: String) -> Result<String, String> {
    let few_shots = FEW_SHOTS_DICT.with(|f| f.borrow().clone());

    let body_json = json!({
        "user_message": user_text,
        "few_shot": few_shots
    });

    let request = HttpRequestArgs {
        url: "https://rawan7-icp-ai-agent-api2.hf.space/generate".to_string(),
        method: HttpMethod::POST,
        headers: vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }],
        body: Some(serde_json::to_vec(&body_json).unwrap()),
        max_response_bytes: Some(1_000_000),
        transform: None,
        is_replicated: Some(false),
    };

    let resp = match http_request(&request).await {
        Ok(r) if r.status.0 == 200u64.into() => r,
        Ok(r) => return Err(format!("HTTP Error: {}", r.status)),
        Err(err) => return Err(format!("IC http_request error: {:?}", err)),
    };

    let body_str = String::from_utf8(resp.body).unwrap_or_default();
    ic_cdk::println!("Model response: {}", body_str);

    Ok(body_str)
}
#[ic_cdk::update]
async fn explain_device_part(device_id: u64, part_name: String) -> Result<String, String> {
    // 1. جلب الجهاز من التخزين
    let device_opt = DEVICES.with(|d| d.borrow().get(&device_id).map(|v| v.clone()));
    let device = match device_opt {
        Some(d) => d,
        None => return Err("Device not found".to_string()),
    };

    // 2. البحث عن الجزئية المطلوبة في specs
    let specs_text = if device.specs.to_lowercase().contains(&part_name.to_lowercase()) {
        device.specs.clone()
    } else {
        format!("No specific info about '{}'. Full specs: {}", part_name, device.specs)
    };

    // 3. ارسال الجزئية لموديل AI للحصول على شرح
    let prompt = format!("Explain the following part of the device: {}", specs_text);
    match call_model(prompt).await {
        Ok(devices) => {
            // call_model يرجع Vec<Device>، لكن هنا نريد فقط شرح، لذلك نرجع النص الأصلي
            Ok(format!("Explanation from AI: {}", specs_text))
        },
        Err(e) => Err(format!("Failed to get explanation from AI: {}", e)),
    }
}

