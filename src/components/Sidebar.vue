<script setup lang="ts">
import { ref } from 'vue'
import logoURL from '../assets/logo.png';
import icons from '../assets/icons';

const is_expanded = ref(localStorage.getItem("is_expanded") === "true");

const ToggleMenu = () => {
    is_expanded.value = !is_expanded.value;
    localStorage.setItem("is_expanded", is_expanded.value.toString());
}
</script>

<template>
    <aside :class="`${is_expanded ? 'is-expanded' : ''}`">
        <div class="logo">
            <img :src="logoURL" alt="Vue" />
        </div>

        <div class="menu-toggle-wrap">
            <button class="menu-toggle" @click="ToggleMenu">
                <img :src="icons.arrow_right" class="icon">
            </button>
        </div>

        <h3>Menu</h3>
        <div class="menu">
            <router-link to="/" class="sidebar-button">
                <img :src="icons.home" class="icon">
                <span class="text">Inicio</span>
            </router-link>
            <router-link to="/create" class="sidebar-button">
                <img :src="icons.add" class="icon">
                <span class="text">Crear cuestionario</span>
            </router-link>
            <router-link to="/create-from-file" class="sidebar-button">
                <img :src="icons.add" class="icon">
                <span class="text">Subir un fichero</span>
            </router-link>
            <router-link to="/manage" class="sidebar-button">
                <img :src="icons.settings" class="icon">
                <span class="text">Gestionar cuestionarios</span>
            </router-link>
        </div>
    </aside>
</template>


<style lang="scss" scoped>
button {
	cursor: pointer;
	appearance: none;
	border: none;
	outline: none;
	background: none;
}

aside {
    display: flex;
    flex-direction: column;

    background-color: var(--dark);
    color: var(--light);

    width: calc(2rem + 32px);
    overflow: hidden;
    min-height: 100vh;
    padding: 1rem;

    transition: 0.2s ease-in-out;

    .flex {
        flex: 1 1 0%;
    }

    .logo {
        display: flex;
        width: 100%;
        justify-content: center;
        margin-bottom: 2em;

        img {
            width: 2rem;
        }
    }

    .icon {
        font-size: 2rem;
        filter: invert(100%);
        transition: 0.2s ease-out;
    }

    .menu-toggle-wrap {
        display: flex;
        justify-content: center;

        position: relative;
        top: 0;
        transition: 0.2s ease-in-out;

        .menu-toggle {
            transition: 0.2s ease-in-out;

            &:hover {
                .icon {
                    color: var(--primary);
                    transform: translateX(0.5rem);
                }
            }
        }
    }

    h3,
    .sidebar-button .text {
        opacity: 0;
        transition: opacity 0.3s ease-in-out;
    }

    h3 {
        color: var(--grey);
        font-size: 0.875rem;
        margin-bottom: 0.5rem;
        text-transform: uppercase;
    }

    .menu {
        margin: 0 -1rem;

        .sidebar-button {
            display: flex;
            align-items: center;
            text-decoration: none;

            transition: 0.2s ease-in-out;
            padding: 0.5rem 1rem;

            .icon {
                font-size: 2rem;
                color: var(--light);
                transition: 0.2s ease-in-out;
            }

            .text {
                color: var(--light);
                transition: 0.2s ease-in-out;
            }

            &:hover {
                background-color: var(--dark-alt);

                .icon,
                .text {
                    color: var(--primary);
                }
            }

            &.router-link-exact-active {
                background-color: var(--dark-alt);
                border-right: 5px solid var(--primary);

                .icon,
                .text {
                    color: var(--primary);
                }
            }
        }
    }

    .footer {
        opacity: 0;
        transition: opacity 0.3s ease-in-out;

        p {
            font-size: 0.875rem;
            color: var(--grey);
        }
    }

    &.is-expanded {
        width: var(--sidebar-width);

        .menu-toggle-wrap {

            .menu-toggle {
                transform: rotate(-180deg);
            }
        }

        h3,
        .sidebar-button .text {
            opacity: 1;
        }

        .sidebar-button {
            .icon {
                margin-right: 1rem;
            }
        }

        .footer {
            opacity: 0;
        }
    }

    @media (max-width: 1024px) {
        position: absolute;
        z-index: 99;
    }
}
</style>