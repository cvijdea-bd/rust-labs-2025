.class public Ljava/TestClass1;
.super Ljava/lang/Object;
.source "TestClass1.java"

.field public static staticCounter:I

.field private id:I

.field protected name:Ljava/lang/String;

.method static constructor <clinit>()V
  .registers 1
  .line 6
    const/4 v0, 0
    sput v0, Ljava/TestClass1;->staticCounter:I
    return-void
.end method

.method public constructor <init>(ILjava/lang/String;)V
  .registers 3
  .line 13
    invoke-direct { p0 }, Ljava/lang/Object;-><init>()V
  .line 14
    iput p1, p0, Ljava/TestClass1;->id:I
  .line 15
    iput-object p2, p0, Ljava/TestClass1;->name:Ljava/lang/String;
  .line 16
    return-void
.end method

.method public static incrementCounter()V
  .registers 1
  .line 20
    sget v0, Ljava/TestClass1;->staticCounter:I
    add-int/lit8 v0, v0, 1
    sput v0, Ljava/TestClass1;->staticCounter:I
  .line 21
    return-void
.end method

.method private logInternal()V
  .registers 5
  .line 30
    sget-object v0, Ljava/lang/System;->out:Ljava/io/PrintStream;
    iget v1, p0, Ljava/TestClass1;->id:I
    new-instance v2, Ljava/lang/StringBuilder;
    invoke-direct { v2 }, Ljava/lang/StringBuilder;-><init>()V
    const-string v3, "Internal log for ID: "
    invoke-virtual { v2, v3 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v2
    invoke-virtual { v2, v1 }, Ljava/lang/StringBuilder;->append(I)Ljava/lang/StringBuilder;
    move-result-object v1
    invoke-virtual { v1 }, Ljava/lang/StringBuilder;->toString()Ljava/lang/String;
    move-result-object v1
    invoke-virtual { v0, v1 }, Ljava/io/PrintStream;->println(Ljava/lang/String;)V
  .line 31
    return-void
.end method

.method public displayInfo()V
  .registers 6
  .line 25
    sget-object v0, Ljava/lang/System;->out:Ljava/io/PrintStream;
    iget v1, p0, Ljava/TestClass1;->id:I
    iget-object v2, p0, Ljava/TestClass1;->name:Ljava/lang/String;
    new-instance v3, Ljava/lang/StringBuilder;
    invoke-direct { v3 }, Ljava/lang/StringBuilder;-><init>()V
    const-string v4, "ID: "
    invoke-virtual { v3, v4 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v3
    invoke-virtual { v3, v1 }, Ljava/lang/StringBuilder;->append(I)Ljava/lang/StringBuilder;
    move-result-object v1
    const-string v3, ", Name: "
    invoke-virtual { v1, v3 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v1
    invoke-virtual { v1, v2 }, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    move-result-object v1
    invoke-virtual { v1 }, Ljava/lang/StringBuilder;->toString()Ljava/lang/String;
    move-result-object v1
    invoke-virtual { v0, v1 }, Ljava/io/PrintStream;->println(Ljava/lang/String;)V
  .line 26
    return-void
.end method

.method public final getName()Ljava/lang/String;
  .registers 2
  .line 35
    iget-object v0, p0, Ljava/TestClass1;->name:Ljava/lang/String;
    return-object v0
.end method

.method protected resetName()V
  .registers 2
  .line 40
    const-string v0, "Unnamed"
    iput-object v0, p0, Ljava/TestClass1;->name:Ljava/lang/String;
  .line 41
    return-void
.end method
