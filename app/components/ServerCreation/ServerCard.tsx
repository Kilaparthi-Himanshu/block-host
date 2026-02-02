'use client';

import { isMacAtom, ServerConfig } from "@/app/atoms"
import { useAtomValue } from "jotai";
import { FaCirclePlay } from "react-icons/fa6";
import { IoSettingsSharp } from "react-icons/io5";

export const ServerCard = ({ 
    server
}: {
    server: ServerConfig
}) => {
    const {
        id,
        name,
        version,
        loader,
        ram_gb,
        path,
        created_at,
    } = server;

    const isMac = useAtomValue(isMacAtom);

    return (
        <div className={`w-50 h-65 corner-squircle bg-neutral-800 flex flex-col overflow-hidden cursor-pointer relative ${isMac ? 'rounded-[30px]' : ' rounded-[50px]'}`}>
            <div className="absolute inset-0 flex flex-col pointer-events-none">
                <button 
                    className="flex-1 flex items-center justify-center opacity-0 hover:opacity-100 bg-green-500/90 transition-opacity duration-150 cursor-pointer pointer-events-auto group z-100"
                >
                    <FaCirclePlay size={50} className="text-green-700 group-active:scale-90 transition-[scale]" />
                </button>

                <button 
                    className="flex-1 flex items-center justify-center opacity-0 hover:opacity-100 bg-gray-500/90 transition-opacity duration-150 cursor-pointer pointer-events-auto group z-100"
                >
                    <IoSettingsSharp size={50} className="text-gray-700 group-active:scale-90 transition-[scale]" />
                </button>
            </div>

            <div className="h-full flex flex-col relative">
                <div 
                    className="flex-1 bg-[url('/minecraft.jpg')] bg-cover bg-center mask-[linear-gradient(to_bottom,black_30%,transparent)]
                    [-webkit-mask-image:linear-gradient(to_bottom,black_30%,transparent)]"
                />

                <div className="flex-1 p-2 px-4 flex flex-col justify-center text-white font-semibold font-mono text-sm gap-1">
                    <p>
                        <span className="underline">Name:</span>{" "}
                        {name}
                    </p>
                    <p>
                        <span className="underline">Version:</span>{" "}
                        {version}
                    </p>
                    <p>
                        <span className="underline">Loader:</span>{" "}
                        {loader.charAt(0).toUpperCase() + loader.slice(1)}
                    </p>
                    <p>
                        <span className="underline">RAM:</span>{" "}
                        {ram_gb} GB
                    </p>
                </div>
            </div>
        </div>
    );
}
