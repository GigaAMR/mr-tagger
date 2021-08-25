import { invoke } from '@tauri-apps/api'
import type { event } from '@tauri-apps/api'

export function popup(msg: string) {
  invoke('error_popup', { msg })
}

export async function runCmd<T = unknown>(cmd: string, options: { [key: string]: unknown } = {}) {
  return (await invoke(cmd, options).catch(popup)) as T
}

export function extractUnlistener(futureUnlistener: Promise<event.UnlistenFn>) {
  return async () => {
    const unlisten = await futureUnlistener
    unlisten()
  }
}

type ShortcutOptions = {
  shift?: boolean
  alt?: boolean
  cmdOrCtrl?: boolean
}
const isMac = navigator.userAgent.indexOf('Mac') != -1

function checkModifiers(e: KeyboardEvent | MouseEvent, options: ShortcutOptions) {
  const target = {
    shift: options.shift || false,
    alt: options.alt || false,
    ctrl: (!isMac && options.cmdOrCtrl) || false,
    meta: (isMac && options.cmdOrCtrl) || false,
  }

  const pressed = {
    shift: !!e.shiftKey,
    alt: !!e.altKey,
    ctrl: !!e.ctrlKey,
    meta: !!e.metaKey,
  }

  return (
    pressed.shift === target.shift &&
    pressed.alt === target.alt &&
    pressed.ctrl === target.ctrl &&
    pressed.meta === target.meta
  )
}

export function checkShortcut(e: KeyboardEvent, key: string, options: ShortcutOptions = {}) {
  if (e.key.toUpperCase() !== key.toUpperCase()) return false
  return checkModifiers(e, options)
}
export function checkMouseShortcut(e: MouseEvent, options: ShortcutOptions = {}) {
  return checkModifiers(e, options)
}
