import { LegacyRef, MouseEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFileCirclePlus, faMagnifyingGlass, faRotateRight, faTrash } from "@fortawesome/free-solid-svg-icons";
import { Password, Setter } from "./types";


export interface Props {
    fetchUserData: () => void;
    passwords: Password[];
    setOpenedPassword: Setter<string | null>;
    handleFileCreation: () => void;
    addingPassword: LegacyRef<HTMLDivElement>;
    searchInput: React.RefObject<HTMLInputElement>;
}
export function Sidebar({ fetchUserData, passwords, setOpenedPassword, addingPassword, handleFileCreation, searchInput }: Props) {
    const [displayedPassword, setDisplayedPassword] = useState(passwords ?? []);

    useEffect(() => {
        setDisplayedPassword(passwords.filter(searchFilter) ?? []);
    }, [passwords]);

    function handlePasswordDelete(passwordPath: string) {
        invoke("delete_password", { passwordPath });
    }

    function handleReload(e: MouseEvent<SVGSVGElement, globalThis.MouseEvent>) {
        (e.target as HTMLElement).closest("svg")?.animate([
            {
                transform: "rotate(0deg)"
            },
            {
                transform: "rotate(360deg)"
            }
        ], { duration: 300 });
        fetchUserData();
    }

    function searchFilter(password: Password) {
        let input = searchInput.current?.value.toLowerCase() ?? "";
        if (!input) return true;

        return password.path.toLowerCase().includes(input)
            || Object.keys(password.data).some((v) => v.includes(input))
            || Object.values(password.data).some((v) => v.includes(input));
    }

    return <div className="w-1/3 flex flex-col items-stretch shadow-xl shadow-oxford-blue-950 text-oxford-blue-300 *:border-oxford-blue-400 *:p-2">
        {/* Search bar */}
        <div className="flex flex-row items-center border-y-3">
            <FontAwesomeIcon icon={faMagnifyingGlass} className="opacity-50 pr-2 border-r-2 border-current" />
            <input ref={searchInput} onInput={() => {
                setDisplayedPassword(passwords.filter(searchFilter) ?? []);
            }} type="text" placeholder="Search..." className="text-oxford-blue-200 bg-transparent p-0 pl-2 outline-none border-none shadow-none w-full" />
        </div>

        <div className="border-b-3 !py-1 flex flex-row justify-between">
            <span>Actions</span>
            <div className="*:px-2 cursor-pointer">
                <FontAwesomeIcon icon={faFileCirclePlus} onClick={handleFileCreation} />
                <FontAwesomeIcon icon={faRotateRight} onClick={e => { handleReload(e); }} />
            </div>
        </div>

        {/* Password List */}
        <div className="flex-1 flex flex-col">
            <div ref={addingPassword} className="hidden">
                <input type="text" className="p-1 bg-oxford-blue-900 border-oxford-blue-200 border-2 w-full rounded-none" />
            </div>
            {passwords ?
                displayedPassword.length ?
                    displayedPassword.map((pwd, i) => <div key={i} className="*:cursor-pointer flex justify-between *:opacity-70 hover:*:opacity-100">
                        <span
                            className="w-full"
                            onClick={() => setOpenedPassword(pwd.path)}
                            key={i}
                        >{pwd.path}</span>
                        <FontAwesomeIcon onClick={() => { handlePasswordDelete(pwd.path); fetchUserData(); }} icon={faTrash} />
                    </div>
                    )
                    : <span className="opacity-80">Aucun mot de passe</span>
                : <span>Chargement...</span>}
        </div>
    </div>;
}
