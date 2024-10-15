import { create } from "zustand";

interface DashboardStore {
  addresses: string[];
  activeAddressIndex: number;
  addAddress: (address: string) => void;
  setActiveAddressIndex: (index: number) => void;
}

export const useDashboardStore = create<DashboardStore>((set) => ({
  addresses: ["0x1234567890", "0x0987654321"],
  activeAddressIndex: 0,
  setActiveAddressIndex: (index) => set({ activeAddressIndex: index }),
  addAddress: (address) =>
    set((state) => ({ addresses: [...state.addresses, address] })),
}));
