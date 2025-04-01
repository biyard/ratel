import lottie from "https://cdnjs.cloudflare.com/ajax/libs/lottie-web/5.12.2/lottie.min.js";

// Map to track animation instances
const lottieInstances = new Map();

/**
 * Load and play Lottie animation
 * @param {string} containerId - ID of the container DOM element to display the animation
 * @param {string} animationPath - Lottie JSON file path
 * @param {Object} options - Lottie animation options
 * @returns {Promise} - Resolves when animation is loaded and ready
 */
export async function loadAnimation(containerId, animationPath, options = {}) {
  // Clean up existing animation
  if (lottieInstances.has(containerId)) {
    lottieInstances.get(containerId).destroy();
    lottieInstances.delete(containerId);
  }

  const defaultOptions = {
    loop: true,
    autoplay: true,
    renderer: "svg",
  };

  try {
    // get animation data from the path
    const response = await fetch(animationPath);
    if (!response.ok) {
      throw new Error(`Failed to load animation: ${response.status}`);
    }

    const animationData = await response.json();
    const container = document.getElementById(containerId);

    if (!container) {
      throw new Error(`Container element with ID '${containerId}' not found`);
    }

    // generate animation instance
    const animInstance = lottie.loadAnimation({
      container: container,
      ...defaultOptions,
      ...options,
      animationData,
    });

    lottieInstances.set(containerId, animInstance);
    return Promise.resolve(true);
  } catch (error) {
    console.error("Error loading Lottie animation:", error);
    return Promise.reject(error);
  }
}

/**
 * stops the animation
 * @param {string} containerId
 */
export function stopAnimation(containerId) {
  const anim = lottieInstances.get(containerId);
  if (anim) {
    anim.stop();
  }
}

/**
 * runs the animation
 * @param {string} containerId
 */
export function playAnimation(containerId) {
  const anim = lottieInstances.get(containerId);
  if (anim) {
    anim.play();
  }
}

/**
 * pauses the animation
 * @param {string} containerId
 */
export function pauseAnimation(containerId) {
  const anim = lottieInstances.get(containerId);
  if (anim) {
    anim.pause();
  }
}

/**
 * move in frame
 * @param {string} containerId
 * @param {number} frame - frame number to go to
 */
export function goToFrame(containerId, frame) {
  const anim = lottieInstances.get(containerId);
  if (anim) {
    anim.goToAndStop(frame, true);
  }
}

/**
 * speed of animation
 * @param {string} containerId
 * @param {number} speed
 */
export function setSpeed(containerId, speed) {
  const anim = lottieInstances.get(containerId);
  if (anim) {
    anim.setSpeed(speed);
  }
}

/**
 * delete animation instance
 * @param {string} containerId
 */
export function destroyAnimation(containerId) {
  const anim = lottieInstances.get(containerId);
  if (anim) {
    anim.destroy();
    lottieInstances.delete(containe / rId);
  }
}

/**
 * delete all animation instances
 */
export function destroyAllAnimations() {
  for (const [id, anim] of lottieInstances.entries()) {
    anim.destroy();
    lottieInstances.delete(id);
  }
}
