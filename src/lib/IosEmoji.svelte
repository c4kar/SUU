<script lang="ts">
  interface Props {
    name: string;
    size?: number;
  }

  let { name, size = 26 }: Props = $props();
</script>

<span
  class="ios-emoji"
  style="width: {size}px; height: {size}px;"
  aria-hidden="true"
>
  {#if name === 'water_drop' || name === '💧'}
    <!-- Apple iOS Glossy Water Droplet -->
    <svg viewBox="0 0 64 64" fill="none">
      <defs>
        <radialGradient id="iosDropGrad" cx="35%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#8BE2FF" />
          <stop offset="45%" stop-color="#32ADE6" />
          <stop offset="100%" stop-color="#007AFF" />
        </radialGradient>
      </defs>
      <path
        d="M32 6C32 6 12 29.5 12 43C12 54.05 20.95 60 32 60C43.05 60 52 54.05 52 43C52 29.5 32 6 32 6Z"
        fill="url(#iosDropGrad)"
      />
      <ellipse
        cx="24"
        cy="38"
        rx="4.5"
        ry="9"
        transform="rotate(-18 24 38)"
        fill="#FFFFFF"
        fill-opacity="0.55"
      />
      <circle cx="22" cy="26" r="2.2" fill="#FFFFFF" fill-opacity="0.7" />
    </svg>
  {:else if name === 'water_glass' || name === '🥛'}
    <!-- Apple iOS Tumbler Glass with Fresh Water -->
    <svg viewBox="0 0 64 64" fill="none">
      <defs>
        <linearGradient id="iosGlassWater" x1="0" y1="20" x2="0" y2="58" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stop-color="#76D6FF" />
          <stop offset="100%" stop-color="#1E96E8" />
        </linearGradient>
        <linearGradient id="iosGlassRim" x1="12" y1="8" x2="52" y2="58" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stop-color="#FFFFFF" stop-opacity="0.85" />
          <stop offset="100%" stop-color="#DCE9F2" stop-opacity="0.45" />
        </linearGradient>
      </defs>
      <path d="M14 11H50L45.5 55C45.2 57.5 43.1 59 40.5 59H23.5C20.9 59 18.8 57.5 18.5 55L14 11Z" fill="url(#iosGlassRim)" stroke="#BFD4E3" stroke-width="2" />
      <path d="M16.3 23H47.7L44.5 54C44.3 55.7 42.8 57 41 57H23C21.2 57 19.7 55.7 19.5 54L16.3 23Z" fill="url(#iosGlassWater)" />
      <ellipse cx="32" cy="23" rx="15.7" ry="3.2" fill="#A6E6FF" />
      <path d="M20 15L22.5 52" stroke="#FFFFFF" stroke-width="3" stroke-linecap="round" stroke-opacity="0.65" />
    </svg>
  {:else if name === 'water_mug' || name === '🫗'}
    <!-- Apple iOS Large Water Mug / Pitcher Glass -->
    <svg viewBox="0 0 64 64" fill="none">
      <path d="M44 20H51C54.3 20 57 22.7 57 26V36C57 39.3 54.3 42 51 42H44" stroke="#9BB8CD" stroke-width="4.5" stroke-linecap="round" />
      <rect x="12" y="12" width="34" height="45" rx="7" fill="#E8F3FA" stroke="#B6CFE0" stroke-width="2" />
      <rect x="14.5" y="21" width="29" height="33.5" rx="5" fill="#36A9E1" />
      <path d="M19 16V49" stroke="#FFFFFF" stroke-width="3" stroke-linecap="round" stroke-opacity="0.65" />
    </svg>
  {:else if name === 'water_bottle' || name === '🍼'}
    <!-- Apple iOS Reusable Water Bottle -->
    <svg viewBox="0 0 64 64" fill="none">
      <defs>
        <linearGradient id="iosBottleGrad" x1="18" y1="16" x2="46" y2="58" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stop-color="#62CFF4" />
          <stop offset="100%" stop-color="#2C67F2" />
        </linearGradient>
      </defs>
      <rect x="25" y="5" width="14" height="7" rx="3" fill="#3A4750" />
      <path d="M23 12H41L45 20V53C45 56.3 42.3 59 39 59H25C21.7 59 19 56.3 19 53V20L23 12Z" fill="url(#iosBottleGrad)" />
      <rect x="19" y="29" width="26" height="10" fill="#FFFFFF" fill-opacity="0.22" />
      <path d="M24 21V52" stroke="#FFFFFF" stroke-width="3" stroke-linecap="round" stroke-opacity="0.55" />
    </svg>
  {:else if name === 'tea_ince' || name === '🫖'}
    <!-- Apple iOS Glossy İnce Belli Tea Glass -->
    <svg viewBox="0 0 64 64" fill="none">
      <defs>
        <linearGradient id="iosTeaCrimson" x1="18" y1="18" x2="46" y2="54" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stop-color="#E86F2C" />
          <stop offset="55%" stop-color="#B83314" />
          <stop offset="100%" stop-color="#7A1809" />
        </linearGradient>
      </defs>
      <!-- Saucer -->
      <ellipse cx="32" cy="56" rx="20" ry="4" fill="#E6D5C3" />
      <ellipse cx="32" cy="55" rx="14" ry="2.2" fill="#C9B29B" />
      <!-- Curved waist glass -->
      <path
        d="M19 12H45C45 12 40 24 40 32C40 40 46 47 44 52C42.8 54.5 38 55.5 32 55.5C26 55.5 21.2 54.5 20 52C18 47 24 40 24 32C24 24 19 12 19 12Z"
        fill="#F5F7FA"
        fill-opacity="0.7"
        stroke="#D0DCE5"
        stroke-width="2"
      />
      <path
        d="M20.8 18H43.2C42 23 39.2 27.5 39.2 32C39.2 39.5 44.5 46.5 42.8 51C41.8 53.2 37.5 54 32 54C26.5 54 22.2 53.2 21.2 51C19.5 46.5 24.8 39.5 24.8 32C24.8 27.5 22 23 20.8 18Z"
        fill="url(#iosTeaCrimson)"
      />
      <ellipse cx="32" cy="18" rx="11.2" ry="2.3" fill="#F2994A" />
      <path d="M24 21C26 28 26 36 23 49" stroke="#FFFFFF" stroke-width="2.5" stroke-linecap="round" stroke-opacity="0.55" />
    </svg>
  {:else if name === 'tea_ajda' || name === '🍵'}
    <!-- Apple iOS Large Ajda Tea Glass -->
    <svg viewBox="0 0 64 64" fill="none">
      <defs>
        <linearGradient id="iosAjdaGrad" x1="16" y1="16" x2="48" y2="56" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stop-color="#F28C38" />
          <stop offset="100%" stop-color="#9E2A0E" />
        </linearGradient>
      </defs>
      <path d="M45 23H50C53 23 55 25.5 55 28.5V37.5C55 40.5 53 43 50 43H44" stroke="#CBD5E1" stroke-width="3.5" stroke-linecap="round" />
      <path d="M17 11H47L43 53C42.6 55.5 40.5 57 38 57H26C23.5 57 21.4 55.5 21 53L17 11Z" fill="#F8FAFC" stroke="#CBD5E1" stroke-width="2" />
      <path d="M18.5 18H45.5L42 52C41.7 54 40 55 38 55H26C24 55 22.3 54 22 52L18.5 18Z" fill="url(#iosAjdaGrad)" />
      <path d="M22 21L24 50" stroke="#FFFFFF" stroke-width="2.6" stroke-linecap="round" stroke-opacity="0.5" />
    </svg>
  {:else if name === 'tea_mug'}
    <!-- Apple iOS Warm Tea Mug with Tea Bag Tag -->
    <svg viewBox="0 0 64 64" fill="none">
      <path d="M45 22H51C54.3 22 57 24.7 57 28V37C57 40.3 54.3 43 51 43H45" stroke="#D97706" stroke-width="4.5" stroke-linecap="round" />
      <rect x="13" y="15" width="33" height="41" rx="8" fill="#F59E0B" />
      <rect x="16" y="19" width="27" height="33" rx="5" fill="#B45309" />
      <path d="M35 15V26L39 29" stroke="#FDE68A" stroke-width="2" />
      <rect x="36" y="28" width="7" height="8" rx="1.5" fill="#FEF08A" />
      <path d="M19 22V46" stroke="#FFFFFF" stroke-width="2.5" stroke-linecap="round" stroke-opacity="0.45" />
    </svg>
  {:else if name === 'coffee_turk' || name === '☕'}
    <!-- Apple iOS Turkish Coffee Cup (Fincan) -->
    <svg viewBox="0 0 64 64" fill="none">
      <defs>
        <linearGradient id="iosCupBody" x1="14" y1="22" x2="48" y2="52" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stop-color="#FFFDF9" />
          <stop offset="100%" stop-color="#E6DEC8" />
        </linearGradient>
      </defs>
      <ellipse cx="31" cy="54" rx="22" ry="4.5" fill="#D8CBB8" />
      <path d="M45 27H51C54.3 27 57 29.5 57 32.8C57 36.1 54.3 38.8 51 38.8H44" stroke="#C8B69E" stroke-width="4" stroke-linecap="round" />
      <path d="M15 22H47L44 45C43.2 50 38.8 53 31 53C23.2 53 18.8 50 18 45L15 22Z" fill="url(#iosCupBody)" stroke="#C8B69E" stroke-width="1.8" />
      <!-- Motif Band -->
      <rect x="16.5" y="32" width="29" height="5" fill="#C69C6D" fill-opacity="0.45" />
      <!-- Rich Turkish Coffee Crema -->
      <ellipse cx="31" cy="22.5" rx="15" ry="4" fill="#6F4E37" />
      <ellipse cx="31" cy="22.5" rx="11" ry="2.4" fill="#A47551" />
      <!-- Gentle Steam -->
      <path d="M26 15C26 12 28 11 28 8" stroke="#B8B0A4" stroke-width="2.2" stroke-linecap="round" />
      <path d="M35 15C35 12 37 11 37 8" stroke="#B8B0A4" stroke-width="2.2" stroke-linecap="round" />
    </svg>
  {:else if name === 'coffee_mug' || name === '🥤'}
    <!-- Apple iOS Filter Coffee Mug -->
    <svg viewBox="0 0 64 64" fill="none">
      <path d="M45 22H51C54.5 22 57 24.8 57 28.2V36.8C57 40.2 54.5 43 51 43H45" stroke="#785A46" stroke-width="4.5" stroke-linecap="round" />
      <rect x="13" y="15" width="33" height="41" rx="8" fill="#8D6E63" />
      <rect x="16" y="20" width="27" height="32" rx="5" fill="#4E342E" />
      <ellipse cx="29.5" cy="20" rx="13.5" ry="2.8" fill="#6D4C41" />
      <path d="M19 24V46" stroke="#FFFFFF" stroke-width="2.5" stroke-linecap="round" stroke-opacity="0.35" />
    </svg>
  {:else if name === 'ayran_small' || name === 'ayran_large' || name === '🫙'}
    <!-- Apple iOS Foamy Ayran Glass -->
    <svg viewBox="0 0 64 64" fill="none">
      <path d="M15 14H49L45 54C44.6 56.8 42.3 58.5 39.5 58.5H24.5C21.7 58.5 19.4 56.8 19 54L15 14Z" fill="#F7F9F7" stroke="#CBD8D3" stroke-width="2" />
      <path d="M16.5 22H47.5L44 53C43.7 55 42 56.5 39.5 56.5H24.5C22 56.5 20.3 55 20 53L16.5 22Z" fill="#EFECE6" />
      <!-- Creamy Foam Top -->
      <path d="M15.5 18C15.5 15.5 19 14 23 14.8C25.5 12.8 30.5 12.5 34 14.2C37.5 12.8 42 13.5 44.5 15.2C47.5 15.5 49 17 48.5 19.5C48 21.8 45 23 32 23C19 23 15.5 20.8 15.5 18Z" fill="#FFFFFF" stroke="#DCE5E1" stroke-width="1.5" />
      <!-- Mint leaf accent -->
      <path d="M35 15C38 12 43 13 42 17C39 18 36 17 35 15Z" fill="#52B788" />
    </svg>
  {:else if name === '🔥'}
    <!-- Apple iOS Flame -->
    <svg viewBox="0 0 64 64" fill="none">
      <defs>
        <linearGradient id="iosFlame" x1="32" y1="6" x2="32" y2="58" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stop-color="#FFCC00" />
          <stop offset="55%" stop-color="#FF9500" />
          <stop offset="100%" stop-color="#FF3B30" />
        </linearGradient>
      </defs>
      <path d="M33 6C33 6 49 20 49 37C49 49.15 40.9 58 32 58C23.1 58 15 49.15 15 37C15 27 21 19 24 15C24 21 27 24 27 24C27 15 33 6 33 6Z" fill="url(#iosFlame)" />
      <path d="M32 30C32 30 40 37 40 45C40 51 36.4 55 32 55C27.6 55 24 51 24 45C24 39 32 30 32 30Z" fill="#FFF3B0" />
    </svg>
  {:else if name === '🛡️'}
    <!-- Apple iOS Shield -->
    <svg viewBox="0 0 64 64" fill="none">
      <defs>
        <linearGradient id="iosShield" x1="12" y1="8" x2="52" y2="56" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stop-color="#5AC8FA" />
          <stop offset="100%" stop-color="#007AFF" />
        </linearGradient>
      </defs>
      <path d="M32 7L12 15V31C12 45.5 20.5 54.8 32 58C43.5 54.8 52 45.5 52 31V15L32 7Z" fill="url(#iosShield)" />
      <path d="M32 11V54C40.8 51.2 48 43.3 48 31V17.8L32 11Z" fill="#FFFFFF" fill-opacity="0.18" />
    </svg>
  {:else if name === '📅'}
    <!-- Apple iOS Calendar Icon -->
    <svg viewBox="0 0 64 64" fill="none">
      <rect x="10" y="10" width="44" height="44" rx="11" fill="#FFFFFF" stroke="#D8E0E5" stroke-width="2" />
      <path d="M10 21C10 14.9 14.9 10 21 10H43C49.1 10 54 14.9 54 21V23H10V21Z" fill="#FF3B30" />
      <circle cx="22" cy="34" r="3" fill="#8E8E93" />
      <circle cx="32" cy="34" r="3" fill="#8E8E93" />
      <circle cx="42" cy="34" r="3" fill="#8E8E93" />
      <circle cx="22" cy="44" r="3" fill="#8E8E93" />
      <circle cx="32" cy="44" r="3" fill="#007AFF" />
      <circle cx="42" cy="44" r="3" fill="#8E8E93" />
    </svg>
  {:else if name === '⚙️'}
    <!-- Apple iOS Gear -->
    <svg viewBox="0 0 64 64" fill="none">
      <circle cx="32" cy="32" r="21" fill="#8E8E93" />
      <circle cx="32" cy="32" r="14" fill="#D1D1D6" />
      <circle cx="32" cy="32" r="7" fill="#636366" />
    </svg>
  {:else if name === '🌙'}
    <!-- Apple iOS Crescent Moon -->
    <svg viewBox="0 0 64 64" fill="none">
      <path d="M41 10C28.8 11.5 20 22 20 34.5C20 47.5 30.5 58 43.5 58C48.5 58 53 56.4 56.5 53.6C45.5 53.2 36.8 44.2 36.8 33C36.8 23.4 43.2 15.2 52 12.6C48.6 10.8 44.9 9.8 41 10Z" fill="#FFCC00" />
    </svg>
  {:else if name === '☀️'}
    <!-- Apple iOS Sun -->
    <svg viewBox="0 0 64 64" fill="none">
      <circle cx="32" cy="32" r="15" fill="#FFCC00" />
      <g stroke="#FF9500" stroke-width="4" stroke-linecap="round">
        <line x1="32" y1="6" x2="32" y2="12" />
        <line x1="32" y1="52" x2="32" y2="58" />
        <line x1="6" y1="32" x2="12" y2="32" />
        <line x1="52" y1="32" x2="58" y2="32" />
        <line x1="13.6" y1="13.6" x2="17.8" y2="17.8" />
        <line x1="46.2" y1="46.2" x2="50.4" y2="50.4" />
        <line x1="50.4" y1="13.6" x2="46.2" y2="17.8" />
        <line x1="17.8" y1="46.2" x2="13.6" y2="50.4" />
      </g>
    </svg>
  {:else}
    <!-- Fallback Glossy Circle -->
    <svg viewBox="0 0 64 64" fill="none">
      <circle cx="32" cy="32" r="24" fill="#32ADE6" />
    </svg>
  {/if}
</span>

<style>
  .ios-emoji {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    vertical-align: middle;
    flex-shrink: 0;
    filter: drop-shadow(0 2px 4px rgba(15, 23, 42, 0.1));
  }
  .ios-emoji svg {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>
