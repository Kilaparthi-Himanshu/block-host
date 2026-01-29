'use client';

import { invoke } from "@tauri-apps/api/core";
import { ServerCard } from "./components/ServerCreation/ServerCard";
import { ServerCreateCard } from "./components/ServerCreation/ServerCreateCard";
import ModalRenderer from "./components/ModalRenderer";
import { useEffect, useState } from "react";
import { ServerCreateModal } from "./components/ServerCreation/ServerCreateModal";

export default function Home() {
    const [serverCreateModalOpen, setServerCreateModalOpen] = useState(false);
    const [serverVersions, setServerVersions] = useState<string[] | null>(null);

    const invokeRustCommand = () => {
        invoke('create_server', { name: 'First-Server', version: '1.21.9' })
            .then(result => console.log(result));
    }

    useEffect(() => {
        async function get_versions() {
            try {
                const data = await invoke<string[]>('get_mc_versions');
                setServerVersions(data ?? null);
                console.log(data);
            } catch(err) {
                console.error(err);
            }
        }

        get_versions();
    }, []);

    return (
        <div className="bg-neutral-900 w-full h-full flex p-4 gap-4 flex-wrap overflow-auto app-scroll relative">
            <ServerCreateCard setIsOpen={setServerCreateModalOpen} />
            {/* <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard />
            <ServerCard /> */}
            <ModalRenderer isOpen={serverCreateModalOpen}> {/* This allows for smoother fade out */}
                <ServerCreateModal setIsOpen={setServerCreateModalOpen} versions={serverVersions} />
            </ModalRenderer>
        </div>
    );
}
