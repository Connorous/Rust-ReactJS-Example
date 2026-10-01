import { useEffect, useContext } from "react"
import { WSContext } from "../context/WSContext"

export function useWSEvent(eventName: string, callback: (data: unknown) => void) {
    const {subscribe } = useContext(WSContext);

    useEffect(() => {
        const unsubscribe = subscribe(eventName, callback);
        return unsubscribe;
    }, [eventName]);
}