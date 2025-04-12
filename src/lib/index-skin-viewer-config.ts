import { useLocalStorage } from "@vueuse/core";
import {
  FlyingAnimation,
  IdleAnimation,
  PlayerAnimation,
  RunningAnimation,
  WalkingAnimation,
  WaveAnimation,
} from "skinview3d";
import { computed, Ref, ref } from "vue";
import { Layers } from "vue-skinview3d";

export enum AnimationType {
  idle = "idle",
  walk = "walk",
  run = "run",
  fly = "fly",
  wave = "wave",
  none = "none",
}

const availableAnimations = {
  idle: new IdleAnimation(),
  walk: new WalkingAnimation(),
  run: new RunningAnimation(),
  fly: new FlyingAnimation(),
  wave: new WaveAnimation(),
};

export interface SkinViewerConfig {
  pixelRatioByDevice: number;
  layers: Layers;
  controls: {
    rotate: boolean;
    zoom: boolean;
    pan: boolean;
  };
  light: {
    global: number;
    camera: number;
  };
  autoRotate: {
    enabled: boolean;
    speed: number;
  };
  animation: {
    type: AnimationType;
    speed: number;
    paused: boolean;
  };
  camera: {
    rotation: {
      x: number;
      y: number;
      z: number;
    };
    position: {
      x: number;
      y: number;
      z: number;
    };
    zoom: number;
  };
}

const defaultConfig: SkinViewerConfig = {
  pixelRatioByDevice: 2,
  layers: {
    inner: {
      head: true,
      body: true,
      leftArm: true,
      rightArm: true,
      leftLeg: true,
      rightLeg: true,
    },
    outer: {
      head: true,
      body: true,
      leftArm: true,
      rightArm: true,
      leftLeg: true,
      rightLeg: true,
    },
  },
  controls: {
    rotate: true,
    zoom: false,
    pan: false,
  },
  light: {
    global: 3,
    camera: 0.6,
  },
  autoRotate: {
    enabled: false,
    speed: 2,
  },
  animation: {
    type: AnimationType.walk,
    speed: 0.5,
    paused: false,
  },
  camera: {
    rotation: {
      x: 0,
      y: 0,
      z: 0,
    },
    position: {
      x: 30.5,
      y: 22.0,
      z: 42.0,
    },
    zoom: 1.5,
  },
};

const skinViewerConfigs = useLocalStorage<Record<string, SkinViewerConfig>>(
    "skinViewerConfigs",
    {},
);

export const configMap = new Map<string, Ref<SkinViewerConfig>>();

export function getSkinViewerConfig(id: string): Ref<SkinViewerConfig> {
  if (!configMap.has(id)) {
    const config = ref<SkinViewerConfig>(
        skinViewerConfigs.value[id]
            ? skinViewerConfigs.value[id]
            : JSON.parse(JSON.stringify(defaultConfig)),
    );

    configMap.set(id, config);
    return config;
  }

  return configMap.get(id)!;
}

export function getAnimation(id: string): Ref<PlayerAnimation | null> {
  const config = getSkinViewerConfig(id);

  return computed(() => {
    if (config.value.animation.type === AnimationType.none) {
      return null;
    }

    const animationClass = availableAnimations[config.value.animation.type];

    if (animationClass) {
      animationClass.speed = config.value.animation.speed;
      animationClass.paused = config.value.animation.paused;
      return animationClass;
    }

    return null;
  });
}

