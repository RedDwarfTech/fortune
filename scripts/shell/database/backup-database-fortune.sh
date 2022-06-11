#!/usr/bin/env bash

# 当使用未初始化的变量时，程序自动退出
# 也可以使用命令 set -o nounset
set -u

# 当任何一行命令执行失败时，自动退出脚本
# 也可以使用命令 set -o errexit
set -e

set -x

/opt/homebrew/bin/pg_dump -v -h reddwarf-postgresql.reddwarf-storage.svc.cluster.local \
-U postgres -p 5432 -d fortune \
--exclude-table-data=article_content \
--exclude-table-data=article \
-f /Users/xiaoqiangjiang/backup/fortune-$(date '+%Y%m%d%H%M%S').sql



