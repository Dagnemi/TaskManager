import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
const invoke = window.__TAURI__.invoke

const createTaskButton = document.querySelector('.create-task');
const taskNameInput = document.querySelector('.create-task-name');
const taskPriorityInput = document.querySelector('.create-task-priority');
const taskContainer = document.getElementById('task-container');
const errorMessage = document.getElementById('error-message');

let tasks = [];

createTaskButton.addEventListener('click', () => {
  console.log("Button clicked");
  const taskName = taskNameInput.value.trim();
  const taskPriority = parseInt(taskPriorityInput.value.trim(), 10);

  console.log("taskName:", taskName);
  console.log("taskPriority:", taskPriority);
  invoke('newTask', {task_name: taskName, priority: taskPriority});

  if (taskName && !isNaN(taskPriority) && taskPriority >= 1 && taskPriority <= 255) {
    console.log("Valid input");

    // Clear any previous error message
    errorMessage.style.display = 'none';
    errorMessage.textContent = '';

    // Add the new task to the tasks array
    const taskId = Date.now(); // Unique ID for each task
    tasks.push({ id: taskId, name: taskName, priority: taskPriority });

    // Sort tasks by priority
    tasks.sort((a, b) => a.priority - b.priority);

    // Render the tasks
    renderTasks();

    // Clear the input fields after adding the task
    taskNameInput.value = '';
    taskPriorityInput.value = '';
  } else {
    // Display an error message
    errorMessage.style.display = 'block';
    errorMessage.textContent = 'Please enter a valid task name and priority (1-255)';
  }
});

function renderTasks() {
  // Clear the task container before rendering
  taskContainer.innerHTML = '';

  // Render each task
  tasks.forEach(task => {
    const taskElement = document.createElement('div');
    taskElement.classList.add('task');
    taskElement.innerHTML = `
      <h1>${task.name}</h1>
      <p>Priority: ${task.priority}</p>
      <button class="delete-task" data-id="${task.id}">Delete</button>
    `;
    taskContainer.appendChild(taskElement);
  });

  // Add event listeners to the delete buttons
  document.querySelectorAll('.delete-task').forEach(button => {
    button.addEventListener('click', (event) => {
      const taskId = parseInt(event.target.getAttribute('data-id'), 10);
      deleteTask(taskId);
    });
  });
}

function deleteTask(taskId) {
  // Remove the task from the tasks array
  tasks = tasks.filter(task => task.id !== taskId);

  // Render the updated tasks
  renderTasks();
}
