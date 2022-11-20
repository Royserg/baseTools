import create from 'zustand';
import { devtools, persist } from 'zustand/middleware';

interface SunGlassState {
  active: boolean;
  setActive: (value: boolean) => void;
  opacity: number;
  setOpacity: (value: number) => void;
}

export const useSunGlassStore = create<SunGlassState>()(
  devtools(
    persist(
      (set) => ({
        active: false,
        setActive: (value) => set((state) => ({ active: value })),
        opacity: 30,
        setOpacity: (value) => set((state) => ({ opacity: value })),
      }),
      {
        name: 'sunGlass-storage',
      }
    )
  )
);
