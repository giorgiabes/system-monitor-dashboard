import { invoke } from "@tauri-apps/api/core";

interface CpuInfo { usage: number; core_count: number; }
interface MemoryInfo { used_mb: number; total_mb: number; percent: number; }
interface ProcessInfo { name: string; cpu: number; memory_mb: number; }

function setBar(id: string, percent: number) {
  const bar = document.getElementById(id)!;
  bar.style.width = `${Math.min(percent, 100)}%`;
  bar.classList.remove("warn", "danger");
  if (percent > 90) bar.classList.add("danger");
  else if (percent > 70) bar.classList.add("warn");
}

async function updateDashboard() {
  const [cpu, mem, procs] = await Promise.all([
    invoke<CpuInfo>("get_cpu_info"),
    invoke<MemoryInfo>("get_memory_info"),
    invoke<ProcessInfo[]>("get_top_processes"),
  ]);

  // CPU
  document.getElementById("cpu-usage")!.textContent = `${cpu.usage.toFixed(1)}%`;
  document.getElementById("cpu-cores")!.textContent = `${cpu.core_count} cores`;
  setBar("cpu-bar", cpu.usage);

  // Memory
  document.getElementById("mem-usage")!.textContent = `${mem.percent.toFixed(1)}%`;
  document.getElementById("mem-total")!.textContent = `${mem.total_mb} MB total`;
  document.getElementById("mem-detail")!.textContent = `${mem.used_mb} MB used`;
  setBar("mem-bar", mem.percent);

  // Processes
  document.getElementById("proc-body")!.innerHTML = procs.map(p => `
    <tr>
      <td>${p.name}</td>
      <td>${p.cpu.toFixed(1)}%</td>
      <td>${p.memory_mb} MB</td>
      <td>
        <div class="mini-track">
          <div class="mini-bar" style="width:${Math.min(p.cpu, 100)}%"></div>
        </div>
      </td>
    </tr>
  `).join("");

  // Timestamp
  document.getElementById("last-updated")!.textContent =
    `Updated ${new Date().toLocaleTimeString()}`;
}

updateDashboard();
setInterval(updateDashboard, 2000);
