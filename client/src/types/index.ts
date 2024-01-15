export interface User {
    name: string
}

export type Setter<T> = React.Dispatch<React.SetStateAction<T>>

export interface Password {
    path: string
    data: Record<string, string>
}

export interface AccountData {
    plugins: Record<string, PluginManifest>
    appdata_path: string
    passwords: Password[]
}

export interface FileTree<T> {
    path: string
    name: string
    children: FileTree<T>[]
}

// function password_hierarchy(passwords: Password[]) {
//     const result: FileTree<Password>[] = [];

//     passwords.forEach((pwd) => {
//         let currentTree = result
//         let currentpath = "."
//         for (const folder of pwd.path.split("/").map(s => s.split("\\")).flat(1)) {
//             currentpath += "/" + folder
//             let parentFolder = currentTree.find(f => f.name == folder)
//             if (!parentFolder) {
//                 parentFolder = currentTree[currentTree.push({ name: folder, path: currentpath, children: [] })]
//             }

//             parentFolder.children.push({ name: folder })

//             currentTree = parentFolder.children
//         }
//     })

//     return result
// }

export interface PluginCommandResPayload {
    data: string | null
    err: string | null
}

export interface PluginManifest {
    name: string,
    author: string,
    icon: string | null,
}