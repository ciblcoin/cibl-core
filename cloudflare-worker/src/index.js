// src/index.js - Cloudflare Worker
export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    // 1. مسیر دریافت قیمت لحظه‌ای (Price Feed)
    if (url.pathname === "/get-price") {
      try {
        const response = await fetch(
          "https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT"
        );
        const data = await response.json();
        
        // برگرداندن قیمت به صورت امن به موبایل
        return new Response(JSON.stringify({
          symbol: "SOL",
          price: data.price,
          timestamp: Date.now()
        }), {
          headers: { 
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "*" // اجازه دسترسی به موبایل
          },
        });
      } catch (error) {
        return new Response("Error fetching price", { status: 500 });
      }
    }

    // 2. مسیر ثبت برنده (فقط با دسترسی خاص)
    if (url.pathname === "/resolve-winner" && request.method === "POST") {
      // در اینجا منطق بررسی قیمت شروع و پایان چالش ۱ دقیقه‌ای قرار می‌گیرد
      // و در نهایت دستور پرداخت به کانتراکت صادر می‌شود.
      return new Response(JSON.stringify({ status: "Processing Result" }));
    }

    return new Response("CiBL Oracle Worker is Live!");
  },
};

import { createClient } from '@supabase/supabase-js';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';

export default {
  // این تابع با Cron Trigger هر یک دقیقه اجرا می‌شود
  async scheduled(event, env, ctx) {
    const supabase = createClient(env.SUPABASE_URL, env.SUPABASE_SERVICE_ROLE_KEY);
    const connection = new Connection(env.SOLANA_RPC_URL);

    // ۱. پیدا کردن چالش‌های فعالی که بیش از ۶۰ ثانیه از شروعشان گذشته
    const { data: activeChallenges } = await supabase
      .from('challenges')
      .select('*')
      .eq('status', 'active')
      .lt('start_time', new Date(Date.now() - 60000).toISOString());

    for (const challenge of activeChallenges) {
      // ۲. دریافت قیمت لحظه‌ای از یک Oracle (مثل Pyth یا CoinGecko)
      const currentPrice = await fetchPrice(challenge.asset_pair);

      // ۳. منطق تشخیص برنده بر اساس قیمت شروع و پایان
      const winnerId = determineWinner(challenge, currentPrice);

      // ۴. ارسال دستور واریز به قرارداد هوشمند سولانا
      const tx = await settleOnChain(challenge, winnerId, env);

      if (tx) {
        // ۵. آپدیت وضعیت در دیتابیس
        await supabase
            .from('challenges')
            .update({ status: 'completed', winner_id: winnerId, end_price: currentPrice })
            .eq('id', challenge.id);
      }
    }
  }
};

async function fetchPrice(asset) {
  // در اینجا آدرس قیمت را از CoinGecko یا Pyth می‌گیریم
  const res = await fetch(`https://api.coingecko.com/api/v3/simple/price?ids=${asset}&vs_currencies=usd`);
  const json = await res.json();
  return json[asset].usd;
}
