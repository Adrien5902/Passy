import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowRightArrowLeft, faFolder, faQuestion, faUser } from "@fortawesome/free-solid-svg-icons";
import { User } from "./types";

interface Props {
    openLoginDialog: () => void;
    currentUser: User;
    pluginDialog: React.RefObject<HTMLDialogElement>
}

export function Header({ openLoginDialog, currentUser, pluginDialog }: Props) {

    return <div className="flex flex-row items-center justify-between">
        <div className="flex justify-between flex-row items-center group cursor-pointer w-[calc(33.333333%+3px)] p-2 border-r-3 border-oxford-blue-400" onClick={openLoginDialog}>
            <div className="flex overflow-hidden text-ellipsis items-center">
                <FontAwesomeIcon icon={faUser} />
                <span className="px-2">{currentUser?.name ?? "Current Username"}</span>
            </div>
            <div className="p-1 transition duration-300 group-hover:rotate-180">
                <FontAwesomeIcon icon={faArrowRightArrowLeft} />
            </div>
        </div>

        <div className="pr-4 *:mx-2 *:cursor-pointer *:border-l-2 first:*:border-l-0 *:border-current *:pl-2 *:ml-0" onClick={() => pluginDialog.current?.showModal()}>
            <span><FontAwesomeIcon icon={faFolder} /> Plugins</span>
            <a href="https://github.com/Adrien5902" target="_blank"><FontAwesomeIcon icon={faQuestion} /> About</a>
        </div>
    </div>;
}
