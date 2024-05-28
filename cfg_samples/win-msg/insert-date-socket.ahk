#Requires AutoHotkey v2.0
#Include <_socket>
#SingleInstance Force
Persistent true

F1::listen_to_Kanata()

g := Gui()
g.OnEvent("close",gui_close1)
g.OnEvent("escape",gui_close1)
g.Add("Edit","vMyEdit Multi w500 h500 ReadOnly","")
g.Show()
gui_close1(*) {
  ExitApp
}
print_gui(str) {
  global g
  AppendText(g["MyEdit"].hwnd,str)
}

listen_to_Kanata() { ; client
  sock := winsock("client",cb_sock,"IPV4")
  sock.Connect("127.0.0.1",1111)
}

on_msg_from_kanata(msg) {
  static msg_time := "print time "
  if        msg == "print_date_short" {
    curtime := FormatTime(,"yy")
  } else if msg == "print date long" {
    curtime := FormatTime(,"dddd MMMM d, yyyy H:mm:ss")
  } else if InStr(msg, msg_time) {
    time_format := StrReplace(msg, msg_time)
    curtime := Trim(FormatTime(,time_format), '"')
  } else if msg == ";;;;;;;;" {
    curtime := ";;;;;;;; = 4268070197446523707"
  } else {
    return
  }
  SetKeyDelay(-1, 0)
  SendText(curtime)
}
on_msgn_from_kanata(msg) {
  if        msg == 1 {
    curtime := FormatTime(,"¦#1¦dddd")
  } else if msg == 2 {
    curtime := FormatTime(,"¦#2¦MMMM")
  } else if msg == 2024 {
    curtime := FormatTime(,"¦#2024¦yyyy")
  } else if msg == 4268070197446523707 {
    curtime := "¦#4268070197446523707=;;;;;;;;¦"
  } else {
    return
  }
  SetKeyDelay(-1, 0)
  SendText(curtime)
}


cb_sock(sock, event, err) {
  global client_data
  dbg_sock("sock.name: " sock.name " / event: " event " / err: " err " ===========================")
  if (sock.name = "client") {
    if (event = "close") {
      print_gui("close`nAddress: " sock.addr "`n" "Port: " sock.port)
      client_data := ""
      sock.close()
    } else if (event = "connect") { ; connection complete, if (err = 0) then it is a success
      print_gui("Client...`r`nConnect Addr: " sock.addr "`r`nConnect Port: " sock.port "`r`n`r`n") ; this does not check for failure
    } else if (event = "write") { ; client ready to send/write
      print_gui("event = write")
    } else if (event = "read") { ; there is data to be read
      buf := sock.Recv() ; ↓ we don't know what to expect, use a proper JSON 'push-msg' messages for that
      msg  := StrGet(buf,"UTF-8") ; read message as a string
      msgn := NumGet(buf,"Int64") ; read message as a number (UInt64 not supported by AHK)
      client_data .= msg
      on_msg_from_kanata( msg )
      on_msgn_from_kanata(msgn)
      print_gui("`r`n" msg)
    }
  }
}

; support funcs
AppendText(EditHwnd, sInput, loc := "bottom") { ; Posted by TheGood: https://autohotkey.com/board/topic/52441-append-text-to-an-edit-control/#entry328342
  insPos := (loc="bottom") ? SendMessage(0x000E, 0     , 0             ,, "ahk_id " EditHwnd) : 0 ; WM_GETTEXTLENGTH
  r1 :=                      SendMessage(0x00B1, insPos, insPos        ,, "ahk_id " EditHwnd)     ; EM_SETSEL - place cursor for insert
  r2 :=                      SendMessage(0x00C2, False , StrPtr(sInput),, "ahk_id " EditHwnd)     ; EM_REPLACESEL - insert text at cursor
}
dbg_sock(_in) { ; AHK v2
  Loop Parse _in, "`n", "`r"
    OutputDebug "AHK: " A_LoopField
}
