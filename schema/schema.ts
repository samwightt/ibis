/**
 * The different types of entries a page can be. A page can only be one type of entry.
 */
type EntryTypes
    = "annotation"
    | "attibute"
    | "binding"
    | "builtin"
    | "callback"
    | "category"
    | "class"
    | "command"
    | "component"
    | "constant"
    | "constructor"
    | "define"
    | "delegate"
    | "diagram"
    | "directive"
    | "element"
    | "entry"
    | "enum"
    | "environment"
    | "error"
    | "event"
    | "exception"
    | "extension"
    | "field"
    | "file"
    | "framework"
    | "function"
    | "global"
    | "guide"
    | "hook"
    | "instance"
    | "instruction"
    | "interface"
    | "keyword"
    | "library"
    | "literal"
    | "macro"
    | "method"
    | "mixin"
    | "modifier"
    | "module"
    | "namespace"
    | "notation"
    | "object"
    | "operator"
    | "option"
    | "package"
    | "parameter"
    | "plugin"
    | "procedure"
    | "property"
    | "protocol"
    | "provider"
    | "provisioner"
    | "query"
    | "record"
    | "resource"
    | "sample"
    | "section"
    | "service"
    | "setting"
    | "shortcut"
    | "statement"
    | "struct"
    | "style"
    | "subroutine"
    | "tag"
    | "test"
    | "trait"
    | "type"
    | "union"
    | "value"
    | "variable"
    | "word"

/**
 * The children of a page, a list of IDs in the same JSON structure.
 * Each ID must exist in the array of pages, otherwise the Docubus document is invalid.
 */
type ChildrenIDArray = string[]

interface FileTypeLocal {
    remote: false
    path: string
}

interface FileTypeRemote {
    remote: true
    url: string
}

interface PageType {
    title: string
    description?: string
    contents: FileTypeLocal | FileTypeRemote
    entryType: EntryTypes
    id: string
    children?: ChildrenIDArray
}

interface InformationType {
    name: string
    description?: string
    icon: FileTypeRemote | FileTypeLocal
    language?: string
}

interface RootType {
    info: InformationType
    pages: PageType[]
}