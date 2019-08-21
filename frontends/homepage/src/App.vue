<template>
  <div class="min-h-screen bg-white text-slate-800 flex flex-col">

    <header class="fixed top-0 left-0 right-0 z-50 border-b border-slate-200 bg-white">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 bg-[#3776AB] rounded-lg flex items-center justify-center">
              <span class="text-white font-bold text-lg">R</span>
            </div>
            <span class="text-xl font-bold text-[#3776AB]">{{ config.app.name }}</span>
          </div>
          
          <nav class="hidden md:flex items-center gap-6">
            <button 
              v-for="item in config.navigation" 
              :key="item.id"
              @click="currentSection = item.id"
              :class="[
                'text-sm font-medium transition-colors duration-200',
                currentSection === item.id ? 'text-[#FFD43B]' : 'text-slate-600 hover:text-[#3776AB]'
              ]"
            >
              {{ item.label }}
            </button>
          </nav>

          <div class="flex items-center gap-4">
            <button class="hidden md:flex items-center gap-2 px-4 py-2 text-sm font-medium text-slate-600 hover:text-[#3776AB] transition-colors">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
              GitHub
            </button>
          </div>
        </div>
      </div>
    </header>

    <main class="relative z-10 flex-1 pt-16">
      <section v-if="currentSection === 'home'" class="py-16">
        <div class="max-w-7xl mx-auto px-6 lg:px-8">
          <div class="text-center max-w-3xl mx-auto mb-16">
            <div class="inline-block px-4 py-2 bg-[#FFD43B] text-slate-900 rounded text-sm font-semibold mb-6">
              {{ config.home.hero.badge }}
            </div>
            <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold leading-tight mb-6">
              <span class="text-[#FFD43B]">Rusty</span> <span class="text-[#3776AB]">Python</span>
            </h1>
            <p class="text-lg text-slate-600 mb-4">
              {{ config.home.hero.subtitle }}
            </p>
            <p class="text-base text-slate-500 max-w-2xl mx-auto leading-relaxed">
              {{ config.home.hero.description }}
            </p>
            
            <div class="flex flex-col sm:flex-row gap-4 justify-center items-center mt-8">
              <button 
                @click="currentSection = 'download'"
                class="px-8 py-3 rounded text-base font-semibold bg-[#FFD43B] text-slate-900 hover:bg-[#e5c035] transition-colors"
              >
                立即安装
              </button>
              <button 
                @click="currentSection = 'playground'"
                class="px-8 py-3 rounded text-base font-semibold border-2 border-[#3776AB] text-[#3776AB] hover:bg-[#3776AB] hover:text-white transition-colors"
              >
                试用 Playground
              </button>
            </div>
          </div>

          <div class="max-w-4xl mx-auto mb-16">
            <div class="bg-slate-50 rounded-lg border border-slate-200 p-8">
              <h3 class="text-xl font-bold text-[#3776AB] mb-3">{{ config.home.features[0].title }}</h3>
              <p class="text-slate-600 mb-6">{{ config.home.features[0].description }}</p>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div v-for="(item, index) in config.home.features[0].items" :key="index" class="bg-white rounded-lg p-5 border border-slate-200">
                  <h4 class="font-semibold text-[#3776AB] mb-2">{{ item.title }}</h4>
                  <p class="text-sm text-slate-500">{{ item.description }}</p>
                </div>
              </div>
            </div>
          </div>

          <div class="max-w-5xl mx-auto">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div v-for="(item, index) in config.home.highlights" :key="index" class="bg-white rounded-lg p-6 border border-slate-200 hover:border-[#3776AB] transition-colors">
                <div class="flex items-start gap-4">
                  <div class="w-12 h-12 bg-[#FFD43B] rounded-lg flex items-center justify-center flex-shrink-0">
                    <span class="text-2xl">{{ item.icon }}</span>
                  </div>
                  <div>
                    <h3 class="text-lg font-bold text-[#3776AB] mb-2">{{ item.title }}</h3>
                    <p class="text-sm text-slate-500 leading-relaxed">{{ item.description }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section v-if="currentSection === 'download'" class="py-16">
        <div class="max-w-4xl mx-auto px-6 lg:px-8">
          <div class="text-center mb-12">
            <h1 class="text-3xl md:text-4xl font-bold text-[#3776AB] mb-4">{{ config.download.title }}</h1>
            <p class="text-slate-600">{{ config.download.description }}</p>
          </div>
          
          <div class="mb-12">
            <div class="bg-slate-50 rounded-lg border border-slate-200 p-6">
              <h2 class="text-lg font-bold text-[#3776AB] mb-4">系统要求</h2>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div v-for="(system, index) in config.download.systemRequirements" :key="index" class="bg-white rounded-lg p-4 border border-slate-200">
                  <h3 class="font-semibold text-slate-800 mb-2">{{ system.os }}</h3>
                  <ul class="text-sm text-slate-500 space-y-1">
                    <li v-for="(req, idx) in system.requirements" :key="idx">{{ req }}</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <div class="space-y-4 mb-12">
            <button 
              v-for="(option, index) in config.download.downloadOptions" 
              :key="index"
              class="group w-full flex items-center gap-4 p-6 bg-white border border-slate-200 rounded-lg hover:border-[#3776AB] transition-colors"
            >
              <div class="w-12 h-12 bg-slate-100 rounded-lg flex items-center justify-center">
                <span class="text-2xl">{{ option.icon }}</span>
              </div>
              <div class="flex-1 text-left">
                <h3 class="text-lg font-bold text-slate-800">{{ option.os }}</h3>
                <p class="text-slate-500 text-sm">{{ option.version }}</p>
                <p class="text-slate-400 text-xs mt-1">{{ option.fileType }}</p>
              </div>
              <div class="text-right">
                <span class="inline-flex items-center px-4 py-2 rounded font-semibold text-sm text-white bg-[#3776AB] group-hover:bg-[#2d5f8a] transition-colors">
                  {{ option.buttonText }}
                </span>
              </div>
            </button>
          </div>
          
          <div class="mb-12">
            <div class="bg-slate-50 rounded-lg border border-slate-200 p-6">
              <h2 class="text-lg font-bold text-[#3776AB] mb-4">安装步骤</h2>
              <div class="space-y-4">
                <div v-for="step in config.download.installationSteps" :key="step.step" class="flex gap-4">
                  <div class="w-7 h-7 bg-[#FFD43B] rounded-full flex items-center justify-center text-slate-900 font-bold text-sm flex-shrink-0">{{ step.step }}</div>
                  <div>
                    <h3 class="font-semibold text-slate-800">{{ step.title }}</h3>
                    <p class="text-sm text-slate-500" v-html="step.description"></p>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <div>
            <div class="bg-slate-50 rounded-lg border border-slate-200 p-6">
              <h2 class="text-lg font-bold text-[#3776AB] mb-4">常见问题</h2>
              <div class="space-y-4">
                <div v-for="(faq, index) in config.download.faq" :key="index">
                  <h3 class="font-semibold text-slate-800 mb-1">{{ faq.question }}</h3>
                  <p class="text-sm text-slate-500">{{ faq.answer }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section v-if="currentSection === 'playground'" class="py-16">
        <div class="max-w-6xl mx-auto px-6 lg:px-8">
          <div class="text-center mb-10">
            <h1 class="text-3xl md:text-4xl font-bold text-[#3776AB] mb-3">{{ config.playground.title }}</h1>
            <p class="text-slate-600">{{ config.playground.description }}</p>
          </div>
          
          <PythonPlayground />
        </div>
      </section>
    </main>

    <footer class="border-t border-slate-200 bg-slate-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8 py-12">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8 mb-8">
          <div class="col-span-2 md:col-span-1">
            <div class="flex items-center gap-2 mb-4">
              <div class="w-8 h-8 bg-[#3776AB] rounded-lg flex items-center justify-center">
                <span class="text-white font-bold text-lg">R</span>
              </div>
              <span class="text-xl font-bold text-[#3776AB]">{{ config.app.name }}</span>
            </div>
            <p class="text-slate-500 text-sm">
              {{ config.app.description }}
            </p>
          </div>
          
          <div v-for="(section, index) in config.footer.sections" :key="index">
            <h4 class="text-slate-800 font-semibold mb-3 text-sm">{{ section.title }}</h4>
            <ul class="space-y-2">
              <li v-for="(link, idx) in section.links" :key="idx"><a :href="link.href" class="text-slate-500 hover:text-[#3776AB] transition-colors text-sm">{{ link.text }}</a></li>
            </ul>
          </div>
        </div>
        
        <div class="pt-8 border-t border-slate-200 flex flex-col md:flex-row justify-between items-center gap-4">
          <p class="text-slate-500 text-sm">
            {{ config.footer.copyright }}
          </p>
          <div class="flex items-center gap-6">
            <a v-for="(link, index) in config.footer.legalLinks" :key="index" :href="link.href" class="text-slate-500 hover:text-[#3776AB] transition-colors text-sm">{{ link.text }}</a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import PythonPlayground from "@/components/PythonPlayground.vue";
import config from "@/config.json";

const currentSection = ref<string>("home");
</script>
