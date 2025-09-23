import realDevices from './realDevices.json'

export type Device = {
  id: number
  name: string
  category: 'smartphones' | 'laptops' | 'tablets' | 'headphones' | 'cameras' | 'monitors' | string
  price: number
  originalPrice: number
  condition: 'excellent' | 'very-good' | 'good' | 'fair' | string
  rating: number
  reviews: number
  image: string
  certified: boolean
  warranty: string
  location: string
  seller?: string
  listedDate?: string
  specs?: Record<string, string>
}

const mapDevice = (d: any, index: number): Device => {
  const baseName = [d.brand, d.model].filter(Boolean).join(' ')
  const price = d.inspection_summary && d.inspection_summary.overall_score ? Math.round((100 - d.inspection_summary.overall_score) * 10 + (d.year ? (2025 - d.year) * 5 : 0)) : 199
  const originalPrice = Math.round(price * 1.2)
  const conditionMap = (s: string | undefined) => {
    if (!s) return 'good'
    const v = s.toLowerCase()
    if (v.includes('excellent') || v.includes('very good') || v.includes('very-good')) return 'excellent'
    if (v.includes('good')) return 'good'
    if (v.includes('fair') || v.includes('degraded')) return 'fair'
    return 'good'
  }

  return {
    id: index + 1,
    name: baseName || String(d.device_id || d.model || 'Unknown Device'),
    category: d.category && d.category.toLowerCase().includes('mobile') ? 'smartphones' : (d.category && d.category.toLowerCase() === 'laptop' ? 'laptops' : (d.category || 'other')),
    price,
    originalPrice,
    condition: conditionMap(d.battery?.status || d.screen?.status || undefined),
    rating: d.inspection_summary?.overall_score ? Math.round((d.inspection_summary.overall_score / 20) * 10) / 10 : 4.0,
    reviews: Math.floor(Math.random() * 300) + 10,
    image: d.image || (d.category && d.category.toLowerCase().includes('mobile') ? 'https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?w=800&h=800&fit=crop' : 'https://images.unsplash.com/photo-1517336714731-489689fd1ca8?w=800&h=800&fit=crop'),
    certified: true,
    warranty: '6 months',
    location: d.inspection_summary?.best_use_cases?.[0] || 'Location Unknown',
    seller: d.identity?.serial_number || undefined,
    listedDate: d.blockchain?.timestamp ? new Date(d.blockchain.timestamp).toISOString().split('T')[0] : undefined,
    specs: {
      year: String(d.year || ''),
      cpu: d.cpu?.model || '',
      ram: d.memory?.ram_gb ? `${d.memory.ram_gb}GB` : (d.memory?.capacity_gb ? `${d.memory.capacity_gb}GB` : ''),
      storage: d.memory?.storage_gb ? `${d.memory.storage_gb}GB` : (d.storage?.capacity_gb ? `${d.storage.capacity_gb}GB` : ''),
    }
  }
}

export const devices: Device[] = (realDevices.devices || []).map(mapDevice)

export const getDeviceById = (id: number) => devices.find(d => d.id === id)
