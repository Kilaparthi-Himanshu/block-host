'use client';

import { useState } from 'react';
import { IoMenu } from "react-icons/io5";
import { HiMiniXMark } from "react-icons/hi2";
import { motion, AnimatePresence } from "framer-motion";

export const Sidebar = () => {
    const [sideOpen, setSideOpen] = useState(false);

    return (
        <nav 
            className={`h-screen ${sideOpen ? 'w-75' : 'w-20'} bg-neutral-950 flex flex-col items-center py-4 relative transition-[width] duration-200 border-r border-r-neutral-500`} 
            // onPointerEnter={() => setSideOpen(true)} 
            // onPointerLeave={() => setSideOpen(false)}
        >
            <AnimatePresence mode="wait">
                {!sideOpen ? (
                    <motion.div
                        key="menu"
                        initial={{ opacity: 0, rotate: -90, scale: 0.8 }}
                        animate={{ opacity: 1, rotate: 0, scale: 1 }}
                        exit={{ opacity: 0, rotate: 90, scale: 0.8 }}
                        transition={{ duration: 0.2 }}
                        className="absolute top-4 left-4 cursor-pointer text-white"
                        onClick={() => setSideOpen(true)}
                    >
                        <IoMenu size={40} />
                    </motion.div>
                    ) : (
                    <motion.div
                        key="close"
                        initial={{ opacity: 0, rotate: -90, scale: 0.8 }}
                        animate={{ opacity: 1, rotate: 0, scale: 1 }}
                        exit={{ opacity: 0, rotate: 90, scale: 0.8 }}
                        transition={{ duration: 0.2 }}
                        className="absolute top-4 left-4 cursor-pointer text-white"
                        onClick={() => setSideOpen(false)}
                    >
                        <HiMiniXMark size={40} />
                    </motion.div>
                )}
            </AnimatePresence>
        </nav>
    )
}
