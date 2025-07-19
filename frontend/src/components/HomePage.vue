
<template>
  <div class="min-h-screen bg-gradient-to-br from-[#0D1B2A] via-[#14213D] to-[#0D1B2A] flex flex-col items-center justify-center p-4 text-white overflow-hidden">
    <div class="absolute inset-0 overflow-hidden -z-0">
      <div class="absolute top-20 left-20 w-72 h-72 bg-gradient-to-r from-[#6FFFE9] to-[#32FFC7] rounded-full opacity-20 blur-3xl animate-float"></div>
      <div class="absolute bottom-20 right-20 w-80 h-80 bg-gradient-to-r from-[#3ED9C9] to-[#6FFFE9] rounded-full opacity-15 blur-3xl animate-float" style="animation-delay: 1.5s;"></div>
       <div class="absolute bottom-1/3 left-1/4 w-96 h-96 bg-gradient-to-r from-[#32FFC7] to-[#3ED9C9] rounded-full opacity-10 blur-3xl animate-float" style="animation-delay: 3s;"></div>
    </div>
    
    <div class="text-center z-10">
      <h1 class="text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-[#6FFFE9] to-[#32FFC7] mb-4 animate-reveal">
        Welcome Home!
      </h1>
    </div>

<div style="width:360px;max-width:100%;"><div style="height:0;padding-bottom:56.11%;position:relative;"><iframe width="360" height="202" style="position:absolute;top:0;left:0;width:100%;height:100%;" frameBorder="0" src="https://imgflip.com/embed/a0q0mr"></iframe></div><p><a href="https://imgflip.com/gif/a0q0mr">via Imgflip</a></p></div>
  </div>
</template>

<script>
export default {
  name: 'HomePage'
}
</script>

<style scoped>
@keyframes float {
  0% { transform: translateY(0px) rotate(0deg); }
  50% { transform: translateY(-20px) rotate(10deg); }
  100% { transform: translateY(0px) rotate(0deg); }
}

.animate-float {
  animation: float 12s ease-in-out infinite;
}

@keyframes reveal {
  0% {
    opacity: 0;
    transform: translateY(30px) scale(0.95);
  }
  100% {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.animate-reveal {
  animation: reveal 1s ease-out forwards;
  opacity: 0;
}
</style>
