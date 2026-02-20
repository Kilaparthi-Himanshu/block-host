import { mcLogsAtom, playitLogsAtom } from "@/app/atoms";
import { getDefaultStore } from "jotai";

export async function resetLogs() {
    const store = getDefaultStore();
    store.set(mcLogsAtom, []);
    store.set(playitLogsAtom, []);
}
