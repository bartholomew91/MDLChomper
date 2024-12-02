<script lang="ts">
    import { onMount } from 'svelte';
    import * as THREE from 'three';
    import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

    let el: HTMLCanvasElement;
    let camera,
        scene,
        renderer,
        controls;

    const animate = () => {
        requestAnimationFrame(animate);
        controls.update();
        renderer.render(scene, camera);
    }

    const resize = () => {
        camera.aspect = el.width / el.height;
        camera.updateProjectionMatrix();
        renderer.setSize(el.width, el.height);
    }

    onMount(async () => {
        scene = new THREE.Scene();
        camera = new THREE.PerspectiveCamera(75, 1, 0.1, 2000);
        camera.position.z = 50;
        camera.position.x = 0;
        renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
        controls = new OrbitControls(camera, el);
        controls.update();
        resize();
        animate();
    });

    window.addEventListener('resize', resize);
</script>

<canvas width="630" height="988" bind:this={el}></canvas>