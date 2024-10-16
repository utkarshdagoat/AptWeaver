import { create } from "zustand";

type SwapStoreState = {
  fromAmountUSD: number;
  toAmountAPT: number;
  swapEnabled: boolean;
  walletConnected: boolean;
  activeAddress: string;
  isLoading: boolean;

  setFromAmountUSD: (fromAmountUSD: number) => void;
  setToAmountAPT: (toAmountUSD: number) => void;
  setSwapEnabled: (swapEnabled: boolean) => void;
  setWalletConnected: (walletConnected: boolean) => void;
  setActiveAddress: (activeAddress: string) => void;
  setIsLoading: (isLoading: boolean) => void;
};

export const useSwapStore = create<SwapStoreState>((set) => ({
  fromAmountUSD: 0,
  toAmountAPT: 0,
  swapEnabled: false,
  walletConnected: false,
  activeAddress: "",
  isLoading: false,

  setFromAmountUSD: (fromAmountUSD: number) => set({ fromAmountUSD }),
  setToAmountAPT: (toAmountAPT: number) => set({ toAmountAPT }),
  setSwapEnabled: (swapEnabled: boolean) => set({ swapEnabled }),
  setWalletConnected: (walletConnected: boolean) => set({ walletConnected }),
  setActiveAddress: (activeAddress: string) => set({ activeAddress }),
  setIsLoading: (isLoading: boolean) => set({ isLoading }),
}));
