import SparkMD5 from "spark-md5";

export function calcOfflineUuid(name: string): string {
  const playerName = `OfflinePlayer:${ name }`;
  const hashedBytes = SparkMD5.hash(playerName, true)
      .split("")
      .map((char) => char.charCodeAt(0));

  hashedBytes[6] = (hashedBytes[6] & 0x0f) | 0x30;
  hashedBytes[8] = (hashedBytes[8] & 0x3f) | 0x80;

  const uuidHexString = Array.from(hashedBytes, (byte) => byte.toString(16).padStart(2, "0")).join("");

  return `
  ${ uuidHexString.slice(0, 8) }-
  ${ uuidHexString.slice(8, 12) }-
  ${ uuidHexString.slice(12, 16) }-
  ${ uuidHexString.slice(16, 20) }-
  ${ uuidHexString.slice(20, 32) }`
      .replace(/ /g, "")
      .replace(/\n/g, "");
}

export function getDefaultSkinUrl(uuidString: string): string {
  const uuidWithoutHyphens = uuidString.replace(/-/g, "");
  const mostSignificantBits = BigInt(`0x${ uuidWithoutHyphens.slice(0, 16) }`);
  const leastSignificantBits = BigInt(`0x${ uuidWithoutHyphens.slice(16, 32) }`);
  const xorResult = mostSignificantBits ^ leastSignificantBits;
  const hashedUUID = BigInt.asIntN(32, (xorResult ^ ((xorResult >> 32n) & 0xffffffffn)) & 0xffffffffn);
  const randomValue = hashedUUID % 18n;
  const adjustedIndex = (randomValue ^ 18n) < 0 && randomValue !== 0n ? randomValue + 18n : randomValue;

  const skinVariants = ["alex", "ari", "efe", "kai", "makena", "noor", "steve", "sunny", "zuri"];
  const skinIndex = Number(adjustedIndex) > 8 ? Number(adjustedIndex) - 9 : Number(adjustedIndex);

  return `/default-skin/${ skinVariants[skinIndex] }-${ Number(adjustedIndex) > 8 ? "wide" : "slim" }.png`;
}