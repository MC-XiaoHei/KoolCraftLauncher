export enum GameType {
  Release,
  Snapshot,
  Old,
  FoolsDay,
}

export enum GameLoader {
  Vanilla,
  Fabric,
  NeoForge,
  Forge,
  LiteLoader,
  Cleanroom,
  OptiFine,
}

export interface MinecraftDir {
  path: string;
  name: string;
}

export interface Game {
  id: string;
  loader: GameLoader;
  type: GameType;
  dir: MinecraftDir;
}

export function getBuiltInGameIcon(game: Game): string {
  switch (game.loader) {
    case GameLoader.Fabric:
      return "/3rd-icon/fabric.png";
    case GameLoader.NeoForge:
      return "/3rd-icon/neoforge.png";
    case GameLoader.Forge:
      return "/minecraft-assets/anvil.png";
    case GameLoader.OptiFine:
      return "/minecraft-assets/copper-bulb.png";
    case GameLoader.LiteLoader:
      return "/minecraft-assets/egg.png";
    case GameLoader.Cleanroom:
      return "/minecraft-assets/white-wool.png";
    case GameLoader.Vanilla:
      switch (game.type) {
        case GameType.Release:
          return "/minecraft-assets/grass-block.png";
        case GameType.Snapshot:
          return "/minecraft-assets/command-block.png";
        case GameType.Old:
          return "/minecraft-assets/cobblestone.png";
        case GameType.FoolsDay:
          return "/minecraft-assets/slime-block.png";
      }
  }
}