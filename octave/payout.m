1;

format("long")

# Load this file with source("decentralpension.m")

# To plot use:
#   plot(months, arrayfun(@(month) pensionMonths(month), months))
function retval = pensionMonths (paidMonths)
  retval = paidMonths**2 / 480
endfunction


# Redistribute ETH of the current month to all pensioner so that a pensioner
# with 480 DPT (1 DPT * 12 month * 40 years) receives a pension that is equal to
# the average payment of this month. All ETH left could be saved. If not enough
# ETH is available to pay the average, just pay a fraction for now.
# Qustions:
#   - In case of to less money, we could use some saved money from the
#     pension. But how much?
#   - Wollen wir mehr als den Durchschnitt zahlen?
#   - Bonuses will only be paid, if enough contributions are made.
# Examples:
#   s = 0
#   c = [
#         1 1 1 1 1 1 1 1 1 1;
#         100 100 100 100 100 100 100 100 100 100
#       ]
#   p = [
#         480 480 480 480 480 480 480 480 480 480;
#         0 0 0 0 0 0 0 0 0 0
#       ]
#   payout(s, c, p)
function [savings_eth, pensions] = payout (savings_eth, contributors, pensioners)
  contributors_eth = contributors(1,:)
  contributors_dpt = contributors(2,:)
  
  pensioners_dpt = pensioners(1,:)
  pensioners_months_received = pensioners(2,:)
  

  total_eth_month = sum (contributors_eth)
  avg_eth_month = mean (contributors_eth)

  # just save all eth if there are no pensioners
  # in the system
  if columns (pensioners) == 0
    savings_eth = savings_eth + total_eth_month
    pensions = []
    return;
  endif

  # redistribute contributions of current month if available
  if (total_eth_month == 0)
    pensions = zeros(size (pensioners_dpt))
  else
    # calc the weighted dpt factor
    total_weighted_dpt = sum (pensioners_dpt / 480)

    # max. eth we could give out for each dpt using contributions of current month
    weighted_dpt_eth_rate = total_eth_month / (total_weighted_dpt * (1 / avg_eth_month))

    # never pay out more than the average
    if weighted_dpt_eth_rate > avg_eth_month
      weighted_dpt_eth_rate = avg_eth_month
    endif

    pensions = (pensioners_dpt / 480) * weighted_dpt_eth_rate
  endif

  # add more eth in case we have savings and we have not payed the average yet
  if savings_eth > 0 && (total_eth_month == 0 || weighted_dpt_eth_rate < avg_eth_month)
    total_dpt = sum (contributors_dpt) + sum (pensioners_dpt)
    
    open_months = 480 * columns(contributors) + sum (480 - pensioners_months_received)
    active_users = columns(contributors) + columns(pensioners)
    avg_open_months = open_months / active_users
    # avg_open_months = 480 # should last for 40 years
    
    total_dpt_eth_rate = savings_eth / (total_dpt * avg_open_months) 

    total_dpt_eth_allowed = total_dpt_eth_rate * pensioners_dpt
    pensions = pensions + total_dpt_eth_allowed
  endif

  savings_eth = savings_eth + total_eth_month - sum (pensions)
endfunction