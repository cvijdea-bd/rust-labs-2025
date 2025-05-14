.class public LHelloWorld;
.super Ljava/lang/Object;
.source "HelloWorld.java"

.method public constructor <init>()V
  .registers 1
  .line 1
    invoke-direct { p0 }, Ljava/lang/Object;-><init>()V
    return-void
.end method

.method public static iterative_factorial(I)I
  .registers 3
  .line 20
    nop
  .line 21
    const/4 v0, 1
    const/4 v1, 2
  :L0
    if-gt v1, p0, :L1
  .line 22
    mul-int v0, v0, v1
  .line 21
    add-int/lit8 v1, v1, 1
    goto :L0
  :L1
  .line 24
    return v0
.end method

.method public static main([Ljava/lang/String;)V
  .registers 6
  .line 4
    sget-object p0, Ljava/lang/System;->out:Ljava/io/PrintStream;
    const-string v0, "Good evening!"
    invoke-virtual { p0, v0 }, Ljava/io/PrintStream;->println(Ljava/lang/String;)V
  .line 6
    nop
  .line 7
    sget-object p0, Ljava/lang/System;->out:Ljava/io/PrintStream;
    const/4 v0, 5
    invoke-static { v0 }, LHelloWorld;->recursive_factorial(I)I
    move-result v1
    new-instance v2, Ljava/lang/StringBuilder;
    invoke-direct { v2 }, Ljava/lang/StringBuilder;-><init>()V
    const-string v3, "Recursive factorial of "
    invoke-virtual { v2, v3 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v2
    invoke-virtual { v2, v0 }, Ljava/lang/StringBuilder;->append(I)Ljava/lang/StringBuilder;
    move-result-object v2
    const-string v3, " is: "
    invoke-virtual { v2, v3 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v2
    invoke-virtual { v2, v1 }, Ljava/lang/StringBuilder;->append(I)Ljava/lang/StringBuilder;
    move-result-object v1
    invoke-virtual { v1 }, Ljava/lang/StringBuilder;->toString()Ljava/lang/String;
    move-result-object v1
    invoke-virtual { p0, v1 }, Ljava/io/PrintStream;->println(Ljava/lang/String;)V
  .line 8
    sget-object p0, Ljava/lang/System;->out:Ljava/io/PrintStream;
    invoke-static { v0 }, LHelloWorld;->iterative_factorial(I)I
    move-result v1
    new-instance v2, Ljava/lang/StringBuilder;
    invoke-direct { v2 }, Ljava/lang/StringBuilder;-><init>()V
    const-string v4, "Iterative factorial of "
    invoke-virtual { v2, v4 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v2
    invoke-virtual { v2, v0 }, Ljava/lang/StringBuilder;->append(I)Ljava/lang/StringBuilder;
    move-result-object v0
    invoke-virtual { v0, v3 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v0
    invoke-virtual { v0, v1 }, Ljava/lang/StringBuilder;->append(I)Ljava/lang/StringBuilder;
    move-result-object v0
    invoke-virtual { v0 }, Ljava/lang/StringBuilder;->toString()Ljava/lang/String;
    move-result-object v0
    invoke-virtual { p0, v0 }, Ljava/io/PrintStream;->println(Ljava/lang/String;)V
  .line 9
    return-void
.end method

.method public static recursive_factorial(I)I
  .registers 2
  .line 12
    const/4 v0, 1
    if-eqz p0, :L1
    if-ne p0, v0, :L0
    goto :L1
  :L0
  .line 15
    add-int/lit8 v0, p0, -1
    invoke-static { v0 }, LHelloWorld;->recursive_factorial(I)I
    move-result v0
    mul-int p0, p0, v0
    return p0
  :L1
  .line 13
    return v0
.end method
