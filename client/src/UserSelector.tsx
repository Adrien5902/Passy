import { faCaretUp, faUser } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Setter, User } from "./types";
import { useState } from "react";

interface Props {
    currentUser: User
    users: User[]
    setSelectedUser: Setter<number>
}

export default function UserSelector({ currentUser, users, setSelectedUser }: Props) {
    const [opened, setOpened] = useState(false)

    return <div className="h-5/12 relative border-oxford-blue-400 border-t-2 p-2 flex justify-between flex-row items-center group cursor-pointer" onClick={() => setOpened(o => !o)}>
        <div className="flex overflow-hidden text-ellipsis">
            <FontAwesomeIcon icon={faUser} />
            <span className="px-2">{currentUser?.name ?? "Current Username"}</span>
        </div>
        <div className={"p-1 transition duration-300 group-hover:-translate-y-1 " + (opened ? "rotate-180" : "")}>
            <FontAwesomeIcon icon={faCaretUp} />
        </div>
        <div className={"absolute rounded w-full bottom-full bg-oxford-blue-900 p-1 left-0 shadow-xl shadow-oxford-blue-950 " + (opened ? "" : "hidden")}>
            {users.map((u, i) =>
                <div className="cursor-pointer m-2 bg-oxford-blue-800 p-2 rounded overflow-hidden text-ellipsis" key={i} onClick={() => setSelectedUser(i)}>{u.name}</div>
            )}
        </div>
    </div >
}