import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { IMessage } from "../models/Message";

export function useFileManager() {
  const data = ref<IMessage[]>([]);
  const count = ref(0);
  const countFails = ref<string[]>([]);
  const isLoading = ref(false);
  const isShowInfoFails = ref(false);
  const daysAgoRef = ref<number | null>(null);

  // Получение количества файлов, используя daysAgo
  const getCount = async (days: number) => {
    isLoading.value = true;
    try {
      count.value = await invoke("count", { daysAgo: days });
    } catch (err) {
      console.error("Ошибка при получении количества:", err);
    } finally {
      isLoading.value = false;
      data.value = []; // Обнуляем данные после выполнения
    }
  };

  // Получение сообщений, используя daysAgo
  const getMessages = async (daysAgo: number) => {
    isLoading.value = true;
    daysAgoRef.value = daysAgo;
    try {
      const [mes, coun] = await Promise.all([
        invoke("get_messages", { daysAgo }),
        invoke("count", { daysAgo }),
      ]);
      data.value = mes as IMessage[];
      count.value = coun as number;
    } catch (err) {
      console.error("Ошибка при получении сообщений:", err);
    } finally {
      isLoading.value = false;
    }
  };

  // Удаление файлов
  const removeFiles = async () => {

    try {
      isLoading.value = true;
      await invoke("remove_files", { daysAgo: daysAgoRef.value });
      alert("Файлы удалены");
      reset();
      countFails.value = [];
    } catch (err) {
      console.error("Ошибка при удалении файлов:", err);
    }
    finally{
      isLoading.value = false;
    }
  };

  // Сброс данных
  const reset = () => {
    isLoading.value = false;
    data.value = [];
    count.value = 0;
    countFails.value = [];
    isShowInfoFails.value = false;
    daysAgoRef.value = null;
  };

  // Тоггл для отображения ошибок
  const toggleFails = () => {
    isShowInfoFails.value = !isShowInfoFails.value;
  };

  const gridData = computed(() =>
    data.value.map((e) => ({
      "File Name": e.messagefilename,
      "Created time": e.messagecreatetime,
    }))
  );

  const countData = computed(() => count.value);

  return {
    data,
    count,
    countFails,
    isLoading,
    isShowInfoFails,
    daysAgoRef,
    getCount,
    getMessages,
    removeFiles,
    reset,
    toggleFails,
    gridData,
    countData,
  };
}
