import { RefObject } from "react";
import { PluginManifest } from "./types";

const defaultPluginIcon = "icon.png"

interface Props {
    dialogRef: RefObject<HTMLDialogElement>,
    plugins: Record<string, PluginManifest>
}

export default function PluginDialog({ plugins, dialogRef }: Props) {
    return <dialog ref={dialogRef}>
        {Object.keys(plugins).map((pluginId, i) => {
            const plugin = plugins[pluginId]

            return <div key={i} className="w-48 rounded-xl overflow-hidden bg-oxford-blue-900 cursor-pointer">
                <img src={plugin.icon ?? defaultPluginIcon} className="w-full aspect-square" />
                <div className="p-2 flex flex-col">
                    <span>{plugin.name}</span>
                    <span className="opacity-50 text-sm">{plugin.author}</span>
                </div>
            </div>
        })}
    </dialog>;
}