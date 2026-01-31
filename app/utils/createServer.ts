import { invoke } from "@tauri-apps/api/core";
import { LoaderType } from "../components/ServerCreation/ServerCreateModal";

type CreateServerInput = {
    name: string;
    version: string;
    loader: LoaderType;
    ramGB: number;
}

export async function createServer({
    name,
    version,
    loader,
    ramGB
}: CreateServerInput) {
    if (!name) {
        throw new Error("Server Instance Name Is Required!");
    }

    if (!version) {
        throw new Error("Server Version Is Required!");
    }

    if (!loader) {
        throw new Error("Loader Type Is Required!");
    }

    if (!ramGB) {
        throw new Error("Invalid RAM Allocation");
    }

    const res = await invoke('create_server', {
        name,
        version,
        loader
    });

    console.log(res);
    console.log(name, version, loader, ramGB);
}
