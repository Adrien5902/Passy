import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { AccountData, PluginCommandResPayload, User } from "./types";
import { LoginDialog } from "./LoginDialog";
import { PasswordEdit } from "./PasswordEdit";
import { Sidebar } from "./Sidebar";
import { Header } from "./Header";
import { emit, listen } from "@tauri-apps/api/event";
import PluginDialog from "./PluginDialog";

const controls = {
    search: "KeyS",
    new_file: "KeyN",
}

type Control = keyof typeof controls

function App() {
    const [users, setUsers] = useState<User[]>([])
    const [selectedUser, setSelectedUser] = useState(0)
    const [data, setData] = useState<AccountData | null>(null)
    const [openedPassword, setOpenedPassword] = useState<string | null>(null)

    const dialog = useRef<HTMLDialogElement>(null);
    const searchInput = useRef<HTMLInputElement>(null)
    const addingPassword = useRef<HTMLDivElement>(null)
    const pluginDialog = useRef<HTMLDialogElement>(null)

    const currentUser = users[selectedUser]
    const plugins = data?.plugins ?? {}
    const appdata = data?.appdata_path ?? ""

    function updateData(d: AccountData | null) {
        setData(d)
    }

    useEffect(() => {
        invoke("get_users")
            .then((data) => {
                setUsers(data as User[])

                invoke("login", { username: "Adrien", password: "" })
                    .then(() => {
                        fetchUserData()
                    }).catch(err => { throw err })
            })
            .catch((err) => {
                console.error(err);
            })
    }, [])

    function fetchUserData() {
        invoke("get_user_data")
            .then((data) => {
                updateData(data as AccountData)
            }).catch(err => { throw err })
    }

    function handleFileCreation() {
        const input = addingPassword.current?.querySelector("input") as HTMLInputElement
        if (!input) return

        addingPassword.current?.classList.remove("hidden")

        input.focus()

        function handleInput() {
            if (!input.value) return

            const filePath = input.value

            invoke("create_password", { path: filePath }).then(() => {
                unfocus()
                fetchUserData()
            }).catch(a => console.error(a))
        }

        input.addEventListener("focusout", unfocus)
        input.addEventListener("change", handleInput)

        function unfocus() {
            addingPassword.current?.classList.add("hidden")
            input.value = ""
            input.removeEventListener("focusout", unfocus)
            input.removeEventListener("change", handleInput)
        }
    }

    useEffect(() => {
        window.addEventListener("keypress", (e) => {
            const control = e.ctrlKey && Object.keys(controls)[Object.values(controls).findIndex(k => k == e.code)] as Control

            switch (control) {
                case "search":
                    searchInput.current?.focus()
                    break;

                case "new_file":
                    handleFileCreation()
                    break;

                default:
                    break;
            }
        })

        //@ts-ignore
        window.plugin_invoke = (plugin, command, data = null) => {
            return new Promise((resolve, reject) => {
                try {
                    let json_data = JSON.stringify(data)
                    listen("plugin_res", (e) => {
                        let payload = e.payload as PluginCommandResPayload
                        if (payload.err) {
                            reject(payload.err)
                        } else if (payload.data) {
                            try {
                                let res = JSON.parse(payload.data)
                                resolve(res)
                            } catch (error) {
                                reject(error)
                            }
                        } else {
                            reject()
                        }
                    })
                    emit("plugin", { plugin, command, data: json_data })
                } catch (error) {
                    reject(error)
                }
            })
        }
    }, [])

    function openLoginDialog() {
        dialog.current?.showModal()
    }

    return <div className="relative h-screen w-screen flex flex-col bg-gradient-to-bl from-oxford-blue-500 to-oxford-blue-950 text-oxford-blue-300">

        <Header {...{ openLoginDialog, currentUser, pluginDialog }} />

        {/* Content */}
        <div className="flex-1 flex flex-row">

            <Sidebar {...{
                addingPassword,
                fetchUserData,
                handleFileCreation,
                searchInput,
                setOpenedPassword,
                passwords: data?.passwords ?? [],
            }} />

            {/* Main */}
            <div className="flex-1 bg-gradient-to-bl bg-black from-oxford-blue-200 to-oxford-blue-500 text-oxford-blue-900">
                {openedPassword != null && data?.passwords ?
                    <PasswordEdit password={data.passwords.find((p) => p.path == openedPassword)} updateData={updateData}></PasswordEdit>
                    : ""}
            </div>
        </div>

        <LoginDialog {...{ dialog, setSelectedUser, users }} />
        <PluginDialog dialogRef={pluginDialog} {...{ plugins }} />

    </div>
}

export default App;
