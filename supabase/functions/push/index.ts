// این کد در سرورهای Supabase اجرا می‌شود
import { serve } from "https://deno.land/std@0.168.0/http/server.ts"

serve(async (req) => {
  const { record } = await req.json() // دیتای جدید از جدول duels

  const message = {
    to: record.winner_expo_token, // توکن اختصاصی گوشی برنده
    sound: 'default',
    title: '💰 You Won!',
    body: `Congratulations! You just won ${record.amount} SOL in the duel.`,
    data: { duelId: record.id },
  }

  await fetch('https://exp.host/--/api/v2/push/send', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(message),
  })

  return new Response(JSON.stringify({ ok: true }), { headers: { "Content-Type": "application/json" } })
})

import * as Notifications from 'expo-notifications';

const registerForPushNotifications = async (userId) => {
  const { status } = await Notifications.requestPermissionsAsync();
  if (status !== 'granted') return;

  const token = (await Notifications.getExpoPushTokenAsync()).data;
  
  // ذخیره توکن در دیتابیس برای استفاده‌های بعدی
  await supabase
    .from('profiles')
    .update({ expo_push_token: token })
    .eq('id', userId);
};
