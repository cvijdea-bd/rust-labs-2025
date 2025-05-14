.class public Ljava/TestClass0;
.super Ljava/lang/Object;
.source "TestClass0.java"

.field final static MAX:I = 1000

.field age:I

.field name:Ljava/lang/String;

.method public constructor <init>()V
  .registers 1
  .line 3
    invoke-direct { p0 }, Ljava/lang/Object;-><init>()V
    return-void
.end method

.method public static foo()I
  .registers 1
  .line 7
    const/16 v0, 1000
    return v0
.end method

.method public greet()V
  .registers 5
  .line 14
    sget-object v0, Ljava/lang/System;->out:Ljava/io/PrintStream;
    iget-object v1, p0, Ljava/TestClass0;->name:Ljava/lang/String;
    new-instance v2, Ljava/lang/StringBuilder;
    invoke-direct { v2 }, Ljava/lang/StringBuilder;-><init>()V
    const-string v3, "Hello, "
    invoke-virtual { v2, v3 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v2
    invoke-virtual { v2, v1 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v1
    const-string v2, "!"
    invoke-virtual { v1, v2 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v1
    invoke-virtual { v1 }, Ljava/lang/StringBuilder;->toString()Ljava/lang/String;
    move-result-object v1
    invoke-virtual { v0, v1 }, Ljava/io/PrintStream;->println(Ljava/lang/String;)V
  .line 15
    return-void
.end method
