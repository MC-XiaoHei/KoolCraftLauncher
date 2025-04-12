import {
  FlyingAnimation,
  IdleAnimation,
  PlayerAnimation,
  RunningAnimation,
  WalkingAnimation,
  WaveAnimation,
} from "skinview3d";
import { Layers } from "vue-skinview3d";

const availableAnimations = {
  idle: new IdleAnimation(),
  walk: new WalkingAnimation(),
  run: new RunningAnimation(),
  fly: new FlyingAnimation(),
  wave: new WaveAnimation(),
};

export const layers = reactive<Layers>({
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
});
export const controls = reactive({
  rotate: true,
  zoom: false,
  pan: false,
});
export const light = reactive({
  global: 3,
  camera: 0.6,
});
export const autoRotate = reactive({
  enabled: false,
  speed: 2,
});
export const animation = reactive({
  type: "walk" as null | keyof typeof availableAnimations,
  speed: 0.5,
  paused: false,
});
export const animationClass = computed<PlayerAnimation | null>(() => {
  const animationClass = animation.type && availableAnimations[animation.type];

  animationClass && (animationClass.speed = animation.speed);
  animationClass && (animationClass.paused = animation.paused);

  return animationClass;
});
export const camera = reactive({
  rotation: {
    x: -0.62,
    y: 0.534,
    z: 0.348,
  },
  position: {
    x: 30.5,
    y: 22.0,
    z: 42.0,
  },
  zoom: 1.5
});