import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { invoke } from '@tauri-apps/api/tauri';

interface Task {
  task_id: string;
  created_at: string;
}

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, CommonModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  title = 'file-test';

  tasks: Record<string, Task> = {};

  createTask() {
    invoke('create_task');
  }

  showTasks() {
    invoke<string>('show_tasks').then((res) => {
      console.log(res);
      if (res) {
        this.tasks = JSON.parse(res);
      }
    });
  }

  saveTasks() {
    invoke('save_tasks');
  }
}
