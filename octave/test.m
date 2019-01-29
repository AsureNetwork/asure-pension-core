# Period: 952, Total Eth: 12.029275499829032, Total Pension Eth: 14355.317283657201, Total DPT: 9099.754715774228, Total Contributor: 5, Total Pensioner: 10, Total Done: 0
# User: 0, Status: Retirement, Wallet: 9032.698282479609, Pension: 1441.526456352929, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 637.1766643178709 + (0)
# User: 1, Status: Retirement, Wallet: 9019.142723829762, Pension: 1472.6231929981063, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 650.921895866985 + (0)
# User: 2, Status: Retirement, Wallet: 9037.92127160854, Pension: 1422.7015972294946, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 628.8557896716508 + (0)
# User: 3, Status: Retirement, Wallet: 9061.540618120858, Pension: 1398.0471567682362, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 617.958151224228 + (0)
# User: 4, Status: Retirement, Wallet: 9026.780546619406, Pension: 1448.3058482183494, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 640.1732588485339 + (0)
# User: 5, Status: Retirement, Wallet: 9028.417132355085, Pension: 1429.0141571629508, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 631.6460373732991 + (0)
# User: 6, Status: Retirement, Wallet: 9056.172274019344, Pension: 1402.0457579884962, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 619.72559390711 + (0)
# User: 7, Status: Retirement, Wallet: 9048.962655506532, Pension: 1412.5640661281104, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 624.3748464165806 + (0)
# User: 8, Status: Retirement, Wallet: 9005.963069695244, Pension: 1495.5282208446376, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 661.04626727553 + (0)
# User: 9, Status: Retirement, Wallet: 9033.862177077539, Pension: 1432.9608299658896, Pension Months Allowed: 480, Pensions Months Received: 472, DPT: 633.3905269042752 + (0)
# User: 10, Status: Run, Wallet: 9036.33820670124, Pension: 0, Pension Months Allowed: 464, Pensions Months Received: 0, DPT: 566.865807738258 + (1.5415073090158895)
# User: 11, Status: Run, Wallet: 9041.272821806011, Pension: 0, Pension Months Allowed: 464, Pensions Months Received: 0, DPT: 563.2563264056738 + (2)
# User: 12, Status: Run, Wallet: 9084.46533110563, Pension: 0, Pension Months Allowed: 464, Pensions Months Received: 0, DPT: 529.8781585793438 + (0.9913906638572276)
# User: 13, Status: Run, Wallet: 9068.168346115412, Pension: 0, Pension Months Allowed: 464, Pensions Months Received: 0, DPT: 548.9030413773526 + (1.8867460623738848)
# User: 14, Status: Run, Wallet: 9067.734145327693, Pension: 0, Pension Months Allowed: 464, Pensions Months Received: 0, DPT: 545.582349867509 + (1.8142285430770246)

source("payout.m")

s = 12.029275499829032
c = [
  1 1 1 1 1;
  566.865807738258 563.2563264056738 529.8781585793438 548.9030413773526 545.582349867509
]
p = [
  637.1766643178709 650.921895866985 628.8557896716508 617.958151224228 640.1732588485339 631.6460373732991 619.72559390711 624.3748464165806 661.04626727553 633.3905269042752;
  472 472 472 472 472 472 472 472 472 472
]

payout(s, c, p)