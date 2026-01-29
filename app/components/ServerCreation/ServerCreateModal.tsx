import { AnimatePresence, motion } from "framer-motion"
import { useEffect, useState } from "react";
import { Select, SelectItem } from "@heroui/react";
import { SelectMenu } from "../SelectMenu";

export const ServerCreateModal = ({ 
    setIsOpen, 
    versions,
}: { 
    setIsOpen: React.Dispatch<React.SetStateAction<boolean>>, 
    versions?: string[] | null
}) => {
    const [mappedVersions, setMappedVersions] = useState<{ key: string, label: string }[]>();
    const [instanceName, setInstanceName] = useState<string | null>(null);
    const [instanceVersion, setInstanceVersion] = useState<string | null>(null);

    useEffect(() => {
        function convertVersionList() {
            const data = versions?.map(ver => ({key: ver, label: ver}));
            setMappedVersions(data);
        }

        convertVersionList();
    }, [versions]);

    return (
        <motion.div 
            className='absolute top-0 left-0 size-full bg-black/70 flex items-center justify-center z-205 text-white p-2 font-sans'
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
            onClick={(e) => {
                e.preventDefault();
                setIsOpen(false);
            }}
        >
            <motion.div 
                className="w-110 h-150 bg-gray-800 corner-squircle rounded-[30px] flex flex-col items-center"
                onClick={(e) => e.stopPropagation()}
                initial={{ y: -10 }}
                animate={{ y: 0 }}
                exit={{ y: -10 }}
                transition={{ duration: 0.2 }}
            >
                <div className="w-full h-max p-4">
                    <span className="text-2xl font-semibold">Create Server</span>
                </div>

                <div className="border-b border-amber-600 w-full" />

                <div className="w-full h-full flex flex-col gap-4 p-4 font-semibold font-mono">
                    <div className="flex flex-col gap-3">
                        <span>Instance Name:</span>

                        <input className="outline-0 border-2 focus:border-amber-500 transition-[border] corner-squircle rounded-[20px] p-2" />
                    </div>

                    <div className="flex flex-col gap-3">
                        <span>Server Version:</span>

                        <SelectMenu items={versions ?? null} setInstanceVersion={setInstanceVersion} />
                    </div>
                </div>
            </motion.div>
        </motion.div>
    )
}
