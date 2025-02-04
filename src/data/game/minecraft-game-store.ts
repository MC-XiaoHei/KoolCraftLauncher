import { Game, MinecraftDir } from "@/data/game/models.ts";

export const useMinecraftGameStore = defineStore("minecraft-game-store", () => {
  const games: Ref<Map<string, Game[]>> = ref(new Map());

  function getGames(dir: MinecraftDir): Game[] {
    return games.value.get(dir.path) || [];
  }

  return {
    games,
    getGames,
  };
});
