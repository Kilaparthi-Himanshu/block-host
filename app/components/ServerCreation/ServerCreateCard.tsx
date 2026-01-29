import { CiCirclePlus } from "react-icons/ci";
import { motion } from "framer-motion";

export const ServerCreateCard = ({ setIsOpen }: { setIsOpen: React.Dispatch<React.SetStateAction<boolean>> }) => {
    return (
        <motion.button 
            whileTap={{ scale: 0.95 }}
            transition={{ duration: 0.15, ease: "easeOut" }}
            className="w-50 h-65 corner-squircle border-3 border-neutral-500 rounded-[50px] flex items-center justify-center cursor-pointer hover:bg-neutral-800 transition-[background]" onClick={() => setIsOpen(true)}
        >
            <CiCirclePlus size={80} className="text-neutral-500" />
        </motion.button>
    )
}
