'use client';

import { mcLogsAtom, playitLogsAtom } from "@/app/atoms";
import { listen } from "@tauri-apps/api/event";
import { useAtom } from "jotai";
import { useEffect, useRef, useState } from "react";

type LogTypes = "mc-log" | "playit-log";

export function TerminalPane({ eventName }: { eventName: LogTypes }) {
    let linesAtom = mcLogsAtom; // initialization for safe fallback

    if (eventName === "mc-log") {
        linesAtom = mcLogsAtom;
    } else if (eventName === "playit-log") {
        linesAtom = playitLogsAtom;
    }

    const [lines, setLines] = useAtom(linesAtom);
    const containerRef = useRef<HTMLDivElement>(null);
    const isFirstRender = useRef(true);
    const shouldAutoScroll = useRef(true);

    useEffect(() => {
        let unlisten: any;

        listen<string>(eventName, (event) => {
            setLines(prev => {
                const updated = [...prev, event.payload];
                if (updated.length > 1000) updated.shift(); // prevent memory blow
                return updated;
            });
        }).then(fn => unlisten = fn);

        return () => {
            if (unlisten) unlisten();
        }
    }, [eventName]);

    useEffect(() => {
        const el = containerRef.current;
        if (!el) return;

        const onScroll = () => {
            const threshold = 10;
            shouldAutoScroll.current =
            el.scrollTop + el.clientHeight >= el.scrollHeight - threshold;
        };

        el.addEventListener('scroll', onScroll);
        return () => el.removeEventListener('scroll', onScroll);
    }, []);

    useEffect(() => {
        const el = containerRef.current;
        if (!el) return;

        if (isFirstRender.current) {
            el.scrollTop = el.scrollHeight;
            isFirstRender.current = false;
            return;
        }

        if (!shouldAutoScroll.current) return;

        el.scrollTo({
            top: el.scrollHeight,
            behavior: 'smooth'
        });
    }, [lines]);

    return (
        <div
            ref={containerRef}
            className="bg-neutral-900 border rounded-xl corner-squircle text-green-400 font-mono text-sm p-2 overflow-y-auto w-full h-full app-scroll min-w-0 flex-1 wrap-break-word whitespace-pre-wrap"
        >
            {lines.map((line, i) => (
                <div key={i}>{line}</div>
            ))}
        </div>
    );
}
