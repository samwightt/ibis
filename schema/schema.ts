/**
 * The different types of entries a page can be. A page can only be one type of entry.
 */
type EntryTypes =
  | "annotation"
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
  | "word";

/**
 * The children of a page, a list of IDs in the same JSON structure.
 * Each ID must exist in the array of pages, otherwise the Ibis document is invalid.
 */
type ChildrenIDArray = string[];

interface PageType {
  /**
   * The title of the page. May only contain alphanumeric characters, dashes, and underscores, and spaces.
   */
  title: string;
  /**
   * A short description of the page. May be used in search results.
   */
  description?: string;
  /**
   * A path to the HTML page from the ibis.json file.
   */
  path: string;
  /**
   * The type of entry the page is.
   */
  entryType: EntryTypes;
  /**
   * An optional array of children IDs.
   */
  children?: ChildrenIDArray;
}

/**
 * The page table contains all pages in the package.
 * Each key (string) mut be the ID of the package, and the value must be the PageType.
 */
interface PageTable {
  [key: string]: PageType;
}

/**
 * Specifies the IDs of the pages that should be in the sidebar. Any page, regardless of whether
 * it's the child of another page or not, may appear in the sidebar. Any page IDs not included in this list
 * and that do not have parents will not be included in the sidebar and will be only accessible from search.
 */
type Sidebar = string[];

interface RootType {
  /**
   * The name of the package. May contain only alphabetic characters, slashes, dashes, and underscores.
   */
  name: string;
  /**
   * The version of the package. NOTE: Version numbers are immutable. Ibis expects that once a package changes,
   * the version number changes as well. This means that you cannot hot-fix content and *must* release a new version
   * of the package when you want to update content.
   */
  version: string;
  /**
   * A description for the package. Will be shown in search results and package management views.
   */
  description?: string;
  /**
   * The author of the package.
   */
  author?: string;
  /**
   * The ISO 639-1 standard language code of the package.
   */
  language?: string;
  /**
   * The page table of the package.
   */
  pages?: PageTable;
  /**
   * The sidebar table of the package.
   */
  sidebar?: Sidebar;
}
