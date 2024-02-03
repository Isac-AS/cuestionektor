<script lang="ts" setup>
import { computed, onMounted, ref } from "vue";
import icons from '../assets/icons/'

// Props for our component,
// these are the same as Notitfication interface.
const props = defineProps({
  id: { type: String, required: true },
  type: {
    type: String,
    default: "info",
    required: false,
  },
  title: { type: String, default: null, required: false },
  message: {
    type: String,
    default: "Ooops! A message was not provided.",
    required: false,
  },
  autoClose: { type: Boolean, default: true, required: false },
  duration: { type: Number, default: 5, required: false },
});

// Defining emits
// for closing a notification
const emit = defineEmits<{
  (e: "close"): void;
}>();

// some reactive values to manage the notification
const timer = ref(-1);
const startedAt = ref<number>(0);
const delay = ref<number>(0);

// setting up the automatic
// dismissing of notificaton
// after the specified duration
onMounted(() => {
  if (props.autoClose) {
    startedAt.value = Date.now();
    delay.value = props.duration * 1000;
    timer.value = setTimeout(close, delay.value);
  }
});

// a computed property to set
// the title of the notification
const toastTitle = computed(() => {
  return props.title && props.title !== null ? props.title : "Notification";
});

// a method to close the
// notification and emit the action
const close = () => {
  emit("close");
};
</script>

<template>
    <Teleport to="body" :ref="id">
        <div 
            :class="`${props.type == 'success' ? 'bg-green-400' : 'bg-wm-error dark:bg-error'}`"
            class="flex rounded fixed bottom-5 right-5 text-OnPrimary p-3 text-lg">
            <img :src="props.type == 'success' ? icons.done : icons.error" class="mr-2 w-6">
            <h1 class="text-lg font-semibold mr-2">{{ toastTitle }}</h1>
            <p>{{ message }}</p>
            <button @click="close" class="ml-3 p-1 hover:brightness-90 bg-inherit rounded">
                <img :src="icons.close" class="w-6">
            </button>
        </div>
    </Teleport>
</template>