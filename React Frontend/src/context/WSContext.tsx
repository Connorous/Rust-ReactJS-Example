import { createContext } from "react";

interface WSContextType {
    subscribe: (eventName: string, callback: (data: unknown) => void) => () => void;
}

export const WSContext = createContext<WSContextType>({} as WSContextType);