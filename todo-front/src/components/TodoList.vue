<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import axios from 'axios'
import { ElMessage } from 'element-plus'

const axiosInstance = axios.create({
  baseURL: 'http://localhost:8080', // 后端的url
  timeout: 5000,
})

const newTodo = ref('');
const todos = ref([])
const categories = ref([])
const selectedCategory = ref('全部');
const selectedCategoryForNewTodo = ref('默认');
const newCategory = ref('');
const showNewCategoryDialog = ref(false)

// 异步获取待办事项
const fetchTodos = async () => {
  try {
    const response = await axiosInstance.get('/api/todos/',
    {
        params: {
          category: selectedCategory.value === '全部' ? "all" : selectedCategory.value,
        },
      }
    )
    todos.value = response.data
    await getAllCategories()
  } catch (error) {
    ElMessage.error('查询待办事项失败')
  }
}

const addTodo = async () => {
  if (newTodo.value.trim() === '') {
    ElMessage.warning('待办事项不能为空');
    return;
  }

  let category = selectedCategoryForNewTodo.value;
  if (category === '新建分类') {
    if (!newCategory.value.trim()) {
      ElMessage.warning('请输入新分类名称');
      return;
    }
    category = newCategory.value.trim();
  }

  try {
    const response = await axiosInstance.post('/api/todos/', {
      title: newTodo.value,
      completed: false,
      category: category
    });
    const newTodoItem = response.data;
    todos.value = [...todos.value, newTodoItem]; // 使用扩展运算符确保视图更新
    newTodo.value = '';
    ElMessage.success('添加待办事项成功');
  } catch (error) {
    ElMessage.error('添加待办事项失败');
    console.error(error);
  }
  await fetchTodos();
};

const toggleComplete = async (todo) => {
  try {
    todo.completed = !todo.completed
    await axiosInstance.put(`/api/todos/${todo.id}`, todo)
    ElMessage.success('待办事项更新成功')
  } catch (error) {
    ElMessage.error('更新待办事项失败')
    console.error(error)
    todo.completed = !todo.completed // 回滚状态
  }
}

const deleteTodo = async (todo) => {
  try {
    await axiosInstance.delete(`/api/todos/${todo.id}`)
    await fetchTodos() // 重新获取待办事项列表
    ElMessage.success('删除待办事项成功')
  } catch (error) {
    ElMessage.error('删除待办事项失败')
    console.error(error)
  }
}

const searchQuery = ref('')

const searchTodos = async () => {
  if (searchQuery.value.trim() === '') {
    ElMessage.warning('请输入搜索关键字')
    return
  }
  let category = selectedCategory.value;
  if (category === '全部') {
    category = 'all';
  }
  try {
    const response = await axiosInstance.get('/api/todos/search', {
      params: { query: searchQuery.value , category: category }
    })
    todos.value = response.data
    if (todos.value.length === 0) {
      ElMessage.info('没有找到相关待办事项')
    } else {
      ElMessage.success('搜索待办事项成功')
    }
  } catch (error) {
    ElMessage.error('搜索待办事项失败')
    console.error(error)
  }
}

const resetTodos = async () => {
  searchQuery.value = ''
  await fetchTodos()
  ElMessage.success('重置搜索成功')
}

const hideCompleted = ref(false)

const filteredTodos = computed(() => {
  if (hideCompleted.value) {
    return todos.value.filter(todo => !todo.completed)
  }
  return todos.value
})

const getAllCategories = async () => {
  try {
    const response = await axiosInstance.get('/api/todos/categories')
    categories.value = response.data
  } catch (error) {
    ElMessage.error('获取分类失败')
    console.error(error)
  }
}

const confirmNewCategory = () => {
  if (!newCategory.value.trim()) return
  categories.value.push(newCategory.value.trim())
  selectedCategoryForNewTodo.value = newCategory.value.trim()
  newCategory.value = ''
  showNewCategoryDialog.value = false
}
// 生命周期钩子函数，组件挂载后执行
onMounted(fetchTodos)
</script>

<template>
<div class="todo-app">
  <el-card class="todo-card">
    <template #header>
      <div class="todo-header">
        待办事项
      </div>
    </template>
    <div class="todo-input">
      <el-input v-model="newTodo" placeholder="新建待办事项..."></el-input>
      <el-select v-model="selectedCategoryForNewTodo" placeholder="选择分类" style="width: 150px;">
        <el-option
          v-for="(category, index) in categories"
          :key="index"
          :label="category"
          :value="category"
        />
        <el-option label="+ 使用新分类" value="newCategory" @click="showNewCategoryDialog = true" />
      </el-select>
      <el-dialog v-model="showNewCategoryDialog" title="新建分类" width="300px">
        <el-input v-model="newCategory" placeholder="输入新分类..." />
        <template #footer>
          <el-button @click="showNewCategoryDialog = false">取消</el-button>
          <el-button type="primary" @click="confirmNewCategory">确定</el-button>
        </template>
      </el-dialog>
      <el-button type="primary" @click="addTodo">添加</el-button>
    </div>
    <div class="search">
      <el-input
        v-model="searchQuery"
        placeholder="搜索待办事项..."
        suffix-icon="el-icon-search"
        clearable
      ></el-input>
      <el-button @click="searchTodos" type="primary">搜索</el-button>
      <el-button @click="resetTodos" type="info">重置</el-button>
    </div>
    <div class="hideCompletedPanel">
      <el-button class="hideCompletedButton" @click="hideCompleted = !hideCompleted"
                 :type="hideCompleted ? 'success' : 'info'">
        {{ hideCompleted ? '显示已完成' : '隐藏已完成' }}
      </el-button>
    </div>
    <div class="categorySelect">
      <el-select v-model="selectedCategory" placeholder="选择分类" style="width: 100%;" @change="fetchTodos">
        <el-option label="全部分类" :value="'全部'"></el-option>
        <el-option
          v-for="(category,index) in categories"
          :key="index"
          :label="category"
          :value="category"
        >
          {{ category }}
        </el-option>
      </el-select>
    </div>
    <div v-if="filteredTodos.length" class="todo-list">
      <el-card v-for="todo in filteredTodos" :key="todo.id" class="todo-item">
        <div class="todo-item-actions">
          <div class="todo-item-title">{{ todo.title }}</div>
          <div class="category-name">
            <span v-if="todo.category">{{ todo.category }}</span>
          </div>
          <el-button class="todo-button" @click="toggleComplete(todo)"
                    :type="todo.completed ? 'success' : 'info' ">
            {{ todo.completed ? '已完成' : '未完成' }}
          </el-button>
          <el-button class="todo-button" @click="deleteTodo(todo)" type="danger">
            删除
          </el-button>
        </div>
      </el-card>
    </div>
    <div v-else class="no-todos"> 暂无待办事项 </div>
  </el-card>
</div>
</template>

<style scoped>
.todo-app{
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  background-color: #f0f2f5;
  padding: 20px;
  box-sizing: border-box;
  font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",
    Arial,sans-serif,"Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol";
}

.todo-card{
  width: 100%;
  max-width: 500px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  background-color: #fff;
}

.todo-header{
  font-size: 24px;
  font-weight: bold;
  text-align: center;
  padding: 16px;
  background-color: #409eff;
  color: #fff;
  border-radius: 8px 8px 0 0;
  margin: 0;
}

.todo-input{
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 20px;
  background-color: #fff;
  border-bottom: 1px solid #ebeef5;
}

.todo-list{
  padding: 20px;
  background-color: #fff;
}

.todo-item{
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 10px 15px;
  margin-bottom: 10px;
  border: 1px solid #ebeef5;
  border-radius: 8px;
  background-color: #f9f9f9;
  transition: background-color 0.3s, transform 0.3s;
}

.todo-item:hover{
  background-color: #e6f7ff;
  transform: translateY(-2px);
}

.todo-item-title{
  font-weight: bold;
  flex: 1;
  margin-right: 5px;
  word-wrap: break-word;
  width: 160px;
}

.completed .todo-item-title{
  text-decoration: line-through;
  color: #909399;
}

.todo-item-actions{
  display: flex;
  align-items: center;
}

.no-todos{
  text-align: center;
  padding: 20px;
  font-size: 18px;
  color: #909399;
}

.search{
  padding: 0 20px;
  display: flex;
  align-items: center;
  margin-bottom: 20px;
  gap: 10px;
}

.hideCompletedPanel{
  padding: 0 20px;
  display: flex;
  align-items: center;
  margin-bottom: 10px;
}

.hideCompletedButton{
  width: 100%;
}

.categorySelect{
  padding: 0 20px;
  margin-bottom: 20px;
}

.category-name{
  font-size: 14px;
  color: #909399;
  margin-right: 20px;
}
</style>
