; 🚀 KURA RUNTIME LIBRARY - LLVM IR (SIMPLIFIED)
; Provides bindings to C standard library for memory management and strings
; All complex logic is done in C - LLVM IR just declares the functions

; ═════════════════════════════════════════════════════════════════
; C Library Declarations (from libc)
; ═════════════════════════════════════════════════════════════════

declare i8* @malloc(i64)              ; Allocate memory
declare void @free(i8*)               ; Free memory  
declare i8* @memcpy(i8*, i8*, i64)    ; Copy memory
declare i8* @memset(i8*, i32, i64)    ; Set memory
declare i64 @strlen(i8*)              ; String length
declare i8* @strcpy(i8*, i8*)         ; String copy
declare i8* @strcat(i8*, i8*)         ; String concatenate
declare i32 @printf(i8*, ...)         ; Print formatted
declare i32 @scanf(i8*, ...)          ; Read formatted
declare void @exit(i32)               ; Exit program

; ═════════════════════════════════════════════════════════════════
; KURA Wrapper Functions
; ═════════════════════════════════════════════════════════════════

; Simple wrapper around malloc for clarity
define i8* @kura_malloc(i64 %size) {
entry:
  %ptr = call i8* @malloc(i64 %size)
  ret i8* %ptr
}

; Simple wrapper around free
define void @kura_free(i8* %ptr) {
entry:
  call void @free(i8* %ptr)
  ret void
}

; Allocate and zero-initialize array
define i8* @kura_calloc(i64 %num, i64 %size) {
entry:
  %total = mul i64 %num, %size
  %ptr = call i8* @malloc(i64 %total)
  %zeroed = call i8* @memset(i8* %ptr, i32 0, i64 %total)
  ret i8* %ptr
}

; ═════════════════════════════════════════════════════════════════
; Array Wrappers (kept simple - actual allocation done in Codegen)
; ═════════════════════════════════════════════════════════════════

; Basic array creation wrapper
define i8* @kura_array_new(i64 %capacity) {
entry:
  %bytes_needed = mul i64 %capacity, 8
  %ptr = call i8* @malloc(i64 %bytes_needed)
  ret i8* %ptr
}

; Free an array
define void @kura_array_delete(i8* %array) {
entry:
  call void @free(i8* %array)
  ret void
}

; ═════════════════════════════════════════════════════════════════
; String Wrappers
; ═════════════════════════════════════════════════════════════════

; Duplicate a string in heap
define i8* @kura_string_dup(i8* %str) {
entry:
  %len = call i64 @strlen(i8* %str)
  %size_needed = add i64 %len, 1
  %ptr = call i8* @malloc(i64 %size_needed)
  %copied = call i8* @strcpy(i8* %ptr, i8* %str)
  ret i8* %ptr
}

; Concatenate two strings (allocates new string)
define i8* @kura_string_concat(i8* %s1, i8* %s2) {
entry:
  %len1 = call i64 @strlen(i8* %s1)
  %len2 = call i64 @strlen(i8* %s2)
  %total_len = add i64 %len1, %len2
  %total_needed = add i64 %total_len, 1
  
  %result = call i8* @malloc(i64 %total_needed)
  %copied1 = call i8* @strcpy(i8* %result, i8* %s1)
  %concatenated = call i8* @strcat(i8* %copied1, i8* %s2)
  ret i8* %result
}

; Free a string
define void @kura_string_free(i8* %str) {
entry:
  call void @free(i8* %str)
  ret void
}

; ═════════════════════════════════════════════════════════════════
; Dictionary Wrappers (simplified)
; ═════════════════════════════════════════════════════════════════

; Create dictionary (simple hash map)
define i8* @kura_dict_new(i64 %capacity) {
entry:
  %data_size = mul i64 %capacity, 16
  %total = add i64 %data_size, 16
  
  %ptr = call i8* @malloc(i64 %total)
  ret i8* %ptr
}

; Free dictionary
define void @kura_dict_free(i8* %dict) {
entry:
  call void @free(i8* %dict)
  ret void
}
