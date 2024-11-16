// src/eventBus.ts
import mitt from "mitt";

// Типы событий, которые будут использоваться
type Events = {
  resetDateSelector: void; // Событие сброса даты
};

const eventBus = mitt<Events>();

export default eventBus;
