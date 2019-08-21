import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { execSync } from "node:child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.join(__dirname, "..");
const isWindows = process.platform === "win32";

const mvpProjects = [
    { name: "hexo-mvp", compiler: "hexo", executable: "hexo", buildArgs: ["generate"] },
    { name: "hugo-mvp", compiler: "hugo", executable: "hugo", buildArgs: ["build"] },
    { name: "jekyll-mvp", compiler: "jekyll", executable: "jekyll", buildArgs: ["build"] },
    { name: "vitepress-mvp", compiler: "vutex", executable: "vutex", buildArgs: ["build"] },
    { name: "vuepress-mvp", compiler: "vutex", executable: "vutex", buildArgs: ["build"] },
];

function getExecutablePath(executable) {
    const exeName = isWindows ? `${executable}.exe` : executable;
    return path.join(rootDir, "target", "debug", exeName);
}

function runCommand(command, cwd) {
    console.log(`Executing: ${command} in ${cwd}`);
    execSync(command, { stdio: "inherit", cwd });
}

async function buildMvpProjects() {
    console.log("Starting MVP build process...");

    try {
        for (const project of mvpProjects) {
            console.log(`\nBuilding ${project.name}...`);
            const projectPath = path.join(rootDir, "examples", project.name);
            const executablePath = getExecutablePath(project.executable);

            if (fs.existsSync(projectPath)) {
                // 检查可执行文件是否存在
                if (fs.existsSync(executablePath)) {
                    // 执行构建命令
                    try {
                        const buildCommand = `${executablePath} ${project.buildArgs.join(" ")}`;
                        runCommand(buildCommand, projectPath);
                        console.log(`\x1b[32m✓ ${project.name} built successfully!\x1b[0m`);
                    } catch (error) {
                        console.log(
                            `\x1b[33m⚠ ${project.name} build failed (this is expected if the compiler isn't fully implemented yet)\x1b[0m`,
                        );
                    }
                } else {
                    console.log(
                        `\x1b[33m⚠ ${project.executable} not found in target/debug. Building the compiler first...\x1b[0m`,
                    );
                    // 尝试构建编译器
                    try {
                        const buildCommand = `cargo build -p ${project.compiler}`;
                        runCommand(buildCommand, rootDir);
                        // 再次尝试构建项目
                        if (fs.existsSync(executablePath)) {
                            const buildCommand = `${executablePath} ${project.buildArgs.join(" ")}`;
                            runCommand(buildCommand, projectPath);
                            console.log(`\x1b[32m✓ ${project.name} built successfully!\x1b[0m`);
                        }
                    } catch (error) {
                        console.log(
                            `\x1b[33m⚠ Failed to build ${project.compiler} compiler (compiler code has errors)\x1b[0m`,
                        );
                    }
                }
            } else {
                console.log(`\x1b[33m⚠ ${project.name} directory not found\x1b[0m`);
            }
        }

        console.log("\n\x1b[32m=== MVP Build Process Complete ===\x1b[0m");
        console.log("Check the examples directories for generated static files.");
    } catch (error) {
        console.error("\n\x1b[31mError during MVP build:\x1b[0m", error);
        process.exit(1);
    }
}

buildMvpProjects();
