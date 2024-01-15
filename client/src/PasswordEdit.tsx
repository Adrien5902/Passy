import { useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { AccountData, Password } from "./types";

export function PasswordEdit({ password, updateData }: { password?: Password; updateData: (d: AccountData | null) => unknown; }) {
    if (!password) return <></>;

    const keyRef = useRef<HTMLInputElement>(null);
    const dataRef = useRef<HTMLInputElement>(null);

    function handleAddKeyToPassword(key: string, data: string) {
        if (!password) return
        password.data[key] = data;
        invoke("update_password", { password });
        invoke("get_user_data").then((d) => {
            updateData(d as AccountData);
        });
    }

    return <div className="m-4">
        <h1 className="text-3xl m-4 text-center">
            {password.path}
        </h1>

        <div>
            {Object.keys(password.data).map((key, i) => <div key={i} className="flex justify-between">
                <span>{key}</span>
                <span>{password.data[key]}</span>
            </div>
            )}
            <div>
                <input type="text" ref={keyRef} />
                <input type="text" ref={dataRef} />
                <button onClick={() => {
                    if (!keyRef.current || !dataRef.current) return;
                    handleAddKeyToPassword(keyRef.current.value, dataRef.current.value);
                    keyRef.current.value = "";
                    dataRef.current.value = "";
                }}>+</button>
            </div>
        </div>
    </div>;
}
