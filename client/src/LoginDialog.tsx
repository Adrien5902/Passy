import { LegacyRef } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowRight, faKey, faUser } from "@fortawesome/free-solid-svg-icons";
import { Setter, User } from "./types";

interface Props {
    setSelectedUser: Setter<number>
    users: User[]
    dialog: LegacyRef<HTMLDialogElement>
}

export function LoginDialog({ setSelectedUser, users, dialog }: Props) {

    return <dialog ref={dialog} className="">
        <form method="dialog" onSubmit={(e) => {
            e.preventDefault();
            // dialog.current?.close()
        }}>
            <div>
                <span><FontAwesomeIcon icon={faUser} /> Account : </span>
                <select required={true} className="glass outline-none" onChange={(e) => setSelectedUser(Number(e.target.value))}>
                    {users.map((u, i) => <option key={i} className="bg-oxford-blue-700 overflow-hidden" value={i}>{u.name}</option>)}
                </select>
            </div>
            <div className="mt-4">
                <span><FontAwesomeIcon icon={faKey} /> Enter password : </span>
                <input required={true} className="glass border-none invalid:text-red-600" type="password" placeholder="Password..." />
            </div>

            <button className="mt-4 hover:translate-x-1 duration-200 transition shadow-lg shadow-oxford-blue-950 text-xl w-full">Next <FontAwesomeIcon icon={faArrowRight} /></button>
        </form>
    </dialog>;
}
