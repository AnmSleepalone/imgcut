<template>
  <!-- <div class="container">
    <div class="row">
      <label for="inputDir">图片目录:</label>
      <input id="inputDir" v-model="inputDir" readonly />
      <button @click="selectInputDir">选择</button>
    </div>
    <div class="row">
      <label for="outputDir">输出目录:</label>
      <input id="outputDir" v-model="outputDir" readonly />
      <button @click="selectOutputDir">选择</button>
    </div>
    <div class="row">
      <label for="originX"> 原点X坐标:</label>
      <input id="originX" v-model="originX" type="number" />
    </div>
    <div class="row">
      <label for="originY">原点Y坐标:</label>
      <input id="originY" v-model="originY" type="number" />
    </div>
    <div class="row">
      <label for="width">裁切宽度:</label>
      <input id="width" v-model="width" type="number" />
    </div>
    <div class="row">
      <label for="height">裁切高度:</label>
      <input id="height" v-model="height" type="number" />
    </div>
    <button @click="confirm">确认</button>
    <div class="row">
      <h3>日志:</h3>
      <pre>{{ log }}</pre>
    </div>
  </div> -->
  <n-form class="container" label-placement="left" label-width="120px">
    <n-form-item label="图片目录">
      <n-input v-model:value="inputDir" placeholder="请选择图片目录">
      </n-input>
      <n-button @click="selectInputDir" type="primary" style="margin-left: 20px;">选择</n-button>
    </n-form-item>
    <n-form-item label="输出目录">
      <n-input v-model:value="outputDir" placeholder="请选择输出目录">
      </n-input>
      <n-button @click="selectOutputDir" type="primary" style="margin-left: 20px;">选择</n-button>
    </n-form-item>
    <n-form-item label="原点X坐标">
      <n-input-number v-model:value="originX" placeholder="请输入原点X坐标" />
    </n-form-item>
    <n-form-item label="原点Y坐标">
      <n-input-number v-model:value="originY" placeholder="请输入原点Y坐标" />
    </n-form-item>
    <n-form-item label="裁切宽度">
      <n-input-number v-model:value="width" placeholder="请输入裁切宽度" />
    </n-form-item>
    <n-form-item label="裁切高度">
      <n-input-number v-model:value="height" placeholder="请输入裁切高度" />
    </n-form-item>
    <n-form-item>
      <n-button @click="confirm" type="primary" block>确认</n-button>
    </n-form-item>
    <div class="log-container">
      <h3>日志:</h3>
      <n-log ref="logInst" :log="log" language="naive-log" trim />
    </div>
  </n-form>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { dialog, event } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/tauri';
import { NForm, NFormItem, NInput, NButton } from 'naive-ui';

const inputDir = ref('');
const outputDir = ref('');
const originX = ref('');
const originY = ref('');
const width = ref('');
const height = ref('');
const log = ref('');


const selectInputDir = async () => {
  try {
    const selected = await dialog.open({
      directory: true,
      multiple: false,
      title: '选择输入目录'
    });
    if (selected) {
      console.log(selected);
      inputDir.value = selected as string;
      console.log(inputDir.value);
    }
  } catch (error) {
    console.error('选择输入目录时出错:', error);
    log.value += `选择输入目录时出错: ${error}\n`;
  }
};

const selectOutputDir = async () => {
  try {
    const selected = await dialog.open({
      directory: true,
      multiple: false,
      title: '选择输出目录'
    });
    if (selected) {
      outputDir.value = selected as string;
      console.log(outputDir);
    }
  } catch (error) {
    console.error('选择输出目录时出错:', error);
    log.value += `选择输出目录时出错: ${error}\n`;
  }
};

const confirm = async () => {
  if (inputDir.value === '' || outputDir.value === '' || originX.value === '' || originY.value === '' || width.value === '' || height.value === '') {
    log.value += '参数不能为空\n';
    console.log(inputDir.value);
    console.log(outputDir.value);
    console.log(originX.value);
    console.log(originY.value);
    console.log(width.value);
    console.log(height.value);
    return;
  }

  try {
    log.value += '开始处理图片...\n';
    await invoke('greet', {
      path: inputDir.value,
      outputDir: outputDir.value,
      x: parseInt(originX.value),
      y: parseInt(originY.value),
      width: parseInt(width.value),
      height: parseInt(height.value)
    });
  } catch (error) {
    console.error('处理图片时出错:', error);
    log.value += `处理图片时出错: ${error}\n`;
  }
};

// 事件监听器
const progressUnlisten = ref<(() => void) | null>(null);
const errorUnlisten = ref<(() => void) | null>(null);
const completedUnlisten = ref<(() => void) | null>(null);

onMounted(async () => {
  progressUnlisten.value = await event.listen('greet-progress', (event) => {
    log.value += `${event.payload as string}\n`;
  });

  errorUnlisten.value = await event.listen('greet-error', (event) => {
    log.value += `错误: ${event.payload as string}\n`;
  });

  completedUnlisten.value = await event.listen('greet-completed', () => {
    log.value += '处理完成\n';
  });
});

onUnmounted(() => {
  if (progressUnlisten.value) progressUnlisten.value();
  if (errorUnlisten.value) errorUnlisten.value();
  if (completedUnlisten.value) completedUnlisten.value();
});
</script>
<style scoped>
.container {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
  background-color: #f9f9f9;
  border-radius: 10px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.log-container {
  margin-top: 20px;
  background-color: #e9ecef;
  padding: 10px;
  border-radius: 5px;
  overflow-x: auto;
}

pre {
  white-space: pre-wrap;
  word-wrap: break-word;
}
/* 滚动槽 */
::-webkit-scrollbar {
    width: 6px;
    height: 6px;
}
::-webkit-scrollbar-track {
    border-radius: 3px;
    background: rgba(0,0,0,0.06);
    -webkit-box-shadow: inset 0 0 5px rgba(0,0,0,0.08);
}
/* 滚动条滑块 */
::-webkit-scrollbar-thumb {
    border-radius: 3px;
    background: rgba(0,0,0,0.12);
    -webkit-box-shadow: inset 0 0 10px rgba(0,0,0,0.2);
}
</style>