interface VersionInterface {
  version: string
  url: string
}

interface PackageInterface {
  name: string
  versions: VersionInterface[]
}

type RootType = PackageInterface[]